use mlua::{Error, Lua, Result, Table, Value};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct LuaDb {
    conn: Arc<Mutex<Connection>>,
}

impl LuaDb {
    fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path).map_err(Error::external)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    fn execute(&self, lua: &Lua, sql: &str, params: Option<Vec<Value>>) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql).map_err(Error::external)?;
        let args: Vec<_> = match params {
            Some(v) => v
                .into_iter()
                .map(|p| lua_value_to_sql(lua, p))
                .collect::<Result<_>>()?,
            None => vec![],
        };
        stmt.execute(rusqlite::params_from_iter(args))
            .map_err(Error::external)?;
        Ok(())
    }

    fn query(&self, lua: &Lua, sql: &str, params: Option<Vec<Value>>) -> Result<Value> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(sql).map_err(Error::external)?;
        let args: Vec<_> = match params {
            Some(v) => v
                .into_iter()
                .map(|p| lua_value_to_sql(lua, p))
                .collect::<Result<_>>()?,
            None => vec![],
        };
        let rows = stmt
            .query_map(rusqlite::params_from_iter(args), |row| {
                let mut table = Vec::new();
                for (i, col) in row.as_ref().column_names().iter().enumerate() {
                    let val: rusqlite::types::Value = row.get(i)?;
                    table.push((col.to_string(), val));
                }
                Ok(table)
            })
            .map_err(Error::external)?;

        let result = lua.create_table()?;
        for (i, r) in rows.enumerate() {
            let row_data = lua.create_table()?;
            for (key, val) in r.map_err(Error::external)? {
                row_data.set(key, rusqlite_value_to_lua(lua, val)?)?;
            }
            result.set(i + 1, row_data)?;
        }
        Ok(Value::Table(result))
    }
}

fn lua_value_to_sql(_lua: &Lua, val: Value) -> Result<rusqlite::types::Value> {
    Ok(match val {
        Value::Nil => rusqlite::types::Value::Null,
        Value::Integer(i) => rusqlite::types::Value::Integer(i),
        Value::Number(n) => rusqlite::types::Value::Real(n),
        Value::String(s) => {
            // Text oder Blob – wir prüfen, ob UTF-8
            match s.to_str() {
                Ok(text) => rusqlite::types::Value::Text(text.to_string()),
                Err(_) => rusqlite::types::Value::Blob(s.as_bytes().to_vec()),
            }
        }
        _ => return Err(Error::external("Unsupported Lua value for SQL parameter")),
    })
}

fn rusqlite_value_to_lua(lua: &Lua, val: rusqlite::types::Value) -> Result<Value> {
    Ok(match val {
        rusqlite::types::Value::Null => Value::Nil,
        rusqlite::types::Value::Integer(i) => Value::Integer(i),
        rusqlite::types::Value::Real(f) => Value::Number(f),
        rusqlite::types::Value::Text(s) => Value::String(lua.create_string(&s)?),
        rusqlite::types::Value::Blob(b) => Value::String(lua.create_string(&b)?),
    })
}

pub fn register(lua: &Lua) -> Result<Table> {
    let sqlite_table = lua.create_table()?;

    let open_fn = lua.create_function(|lua, path: String| {
        let db = LuaDb::new(&path)?;
        let db_table = lua.create_table()?;

        db_table.set("execute", {
            let db = db.clone();
            lua.create_function(move |lua, (sql, params): (String, Option<Vec<Value>>)| {
                db.execute(lua, &sql, params)
            })?
        })?;

        db_table.set("query", {
            let db = db.clone();
            lua.create_function(move |lua, (sql, params): (String, Option<Vec<Value>>)| {
                db.query(lua, &sql, params)
            })?
        })?;

        Ok(db_table)
    })?;

    sqlite_table.set("open", open_fn)?;
    Ok(sqlite_table)
}

#[cfg(test)]
mod tests {
    use mlua::{Lua, Value};

    fn init() -> (Lua, mlua::Table) {
        let lua = Lua::new();
        let sqlite = super::register(&lua).unwrap();
        (lua, sqlite)
    }

    #[test]
    fn create_insert_query_update_delete() {
        let (_lua, sqlite) = init();
        let open = sqlite.get::<mlua::Function>("open").unwrap();
        let db = open.call::<mlua::Table>(":memory:").unwrap();
        let exec = db.get::<mlua::Function>("execute").unwrap();
        let query = db.get::<mlua::Function>("query").unwrap();

        exec.call::<()>("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INT)")
            .unwrap();
        exec.call::<()>("INSERT INTO users (name, age) VALUES ('Bob', 20)")
            .unwrap();

        let rows: Vec<mlua::Table> = query.call("SELECT * FROM users").unwrap();
        assert_eq!(rows.len(), 1);

        exec.call::<()>("UPDATE users SET age=21 WHERE name='Bob'")
            .unwrap();
        let rows: Vec<mlua::Table> = query.call("SELECT age FROM users").unwrap();
        assert_eq!(rows[0].get::<i64>("age").unwrap(), 21);

        exec.call::<()>("DELETE FROM users").unwrap();
        let rows: Vec<mlua::Table> = query.call("SELECT * FROM users").unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn transactions_commit_rollback() {
        let (lua, sqlite) = init();
        let db = sqlite
            .get::<mlua::Function>("open")
            .unwrap()
            .call::<mlua::Table>(":memory:")
            .unwrap();
        let exec = db.get::<mlua::Function>("execute").unwrap();
        let query = db.get::<mlua::Function>("query").unwrap();

        exec.call::<()>("CREATE TABLE items (name TEXT)").unwrap();

        exec.call::<()>("BEGIN").unwrap();
        exec.call::<()>("INSERT INTO items VALUES ('commit1')")
            .unwrap();
        exec.call::<()>("COMMIT").unwrap();

        exec.call::<()>("BEGIN").unwrap();
        exec.call::<()>("INSERT INTO items VALUES ('rollback')")
            .unwrap();
        exec.call::<()>("ROLLBACK").unwrap();

        let rows: Vec<mlua::Table> = query.call("SELECT * FROM items").unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn blob_storage_and_retrieval() -> mlua::Result<()> {
        let (lua, sqlite) = init();
        let db = sqlite
            .get::<mlua::Function>("open")?
            .call::<mlua::Table>(":memory:")?;
        let exec = db.get::<mlua::Function>("execute")?;
        let query = db.get::<mlua::Function>("query")?;

        // Tabelle erstellen
        exec.call::<()>("CREATE TABLE files (data BLOB)")?;

        // Beispiel-Bytes
        let bytes: Vec<u8> = vec![1, 2, 3, 4, 255];
        let lua_blob = lua.create_string(&bytes)?;

        // Einfügen mit Parameterbindung
        exec.call::<()>((
            "INSERT INTO files VALUES (?)",
            Some(vec![Value::String(lua_blob)]),
        ))?;

        // Abrufen
        let rows: Vec<mlua::Table> = query.call("SELECT * FROM files")?;

        // Vergleich
        let result: mlua::String = rows[0].get("data")?;
        assert_eq!(result.as_bytes(), bytes.as_slice());

        Ok(())
    }

    #[test]
    fn constraint_and_error_handling() {
        let (_lua, sqlite) = init();
        let db = sqlite
            .get::<mlua::Function>("open")
            .unwrap()
            .call::<mlua::Table>(":memory:")
            .unwrap();
        let exec = db.get::<mlua::Function>("execute").unwrap();

        exec.call::<()>("CREATE TABLE unique_test (name TEXT UNIQUE)")
            .unwrap();
        exec.call::<()>("INSERT INTO unique_test VALUES ('a')")
            .unwrap();
        assert!(
            exec.call::<()>("INSERT INTO unique_test VALUES ('a')")
                .is_err()
        );
    }

    #[test]
    fn foreign_key_and_pragma() {
        let (lua, sqlite) = init();
        let db = sqlite
            .get::<mlua::Function>("open")
            .unwrap()
            .call::<mlua::Table>(":memory:")
            .unwrap();
        let exec = db.get::<mlua::Function>("execute").unwrap();

        exec.call::<()>("PRAGMA foreign_keys = ON").unwrap();
        exec.call::<()>("CREATE TABLE parent(id INTEGER PRIMARY KEY)")
            .unwrap();
        exec.call::<()>("CREATE TABLE child(pid INTEGER REFERENCES parent(id))")
            .unwrap();

        // should fail because foreign key missing
        assert!(exec.call::<()>("INSERT INTO child VALUES (1)").is_err());
    }

    #[test]
    fn multiple_tables_and_queries() {
        let (lua, sqlite) = init();
        let db = sqlite
            .get::<mlua::Function>("open")
            .unwrap()
            .call::<mlua::Table>(":memory:")
            .unwrap();
        let exec = db.get::<mlua::Function>("execute").unwrap();
        let query = db.get::<mlua::Function>("query").unwrap();

        exec.call::<()>("CREATE TABLE a (val TEXT)").unwrap();
        exec.call::<()>("CREATE TABLE b (num INTEGER)").unwrap();
        exec.call::<()>("INSERT INTO a VALUES ('x'),('y')").unwrap();
        exec.call::<()>("INSERT INTO b VALUES (10),(20)").unwrap();

        let rows_a: Vec<mlua::Table> = query.call("SELECT * FROM a").unwrap();
        let rows_b: Vec<mlua::Table> = query.call("SELECT * FROM b").unwrap();
        assert_eq!(rows_a.len(), 2);
        assert_eq!(rows_b.len(), 2);
    }
}
