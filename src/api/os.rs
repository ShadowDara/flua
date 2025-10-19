use mlua::{Lua, Result, Value};
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use crate::helper::dir::{join_path, secure_path, split_path};

use crate::deprecated;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Function to get OS Type, OS Release, Hostname, CPU Number and Total Memory
    let get_os_info = lua.create_function(|lua, ()| {
        let table = lua.create_table()?;
        table.set("os_type", sys_info::os_type().unwrap_or("Unknown".into()))?;
        table.set(
            "os_release",
            sys_info::os_release().unwrap_or("Unknown".into()),
        )?;
        table.set("hostname", sys_info::hostname().unwrap_or("Unknown".into()))?;
        table.set("cpu_num", sys_info::cpu_num().unwrap_or(0))?;
        table.set(
            "mem_total",
            sys_info::mem_info().map(|m| m.total).unwrap_or(0),
        )?;
        Ok(table)
    })?;

    // Function to check the OS
    let os = lua.create_function(|lua, ()| {
        let table = lua.create_table()?;

        let os_type = sys_info::os_type().map_err(|e| mlua::Error::external(e.to_string()))?;

        let windows = os_type == "Windows";
        let linux = os_type == "Linux";
        let macos = os_type == "Darwin";

        table.set("win", windows)?;
        table.set("lin", linux)?;
        table.set("mac", macos)?;

        Ok(table)
    })?;

    // function to change the current executing directory
    let chdir = lua.create_function(|_, path: String| {
        std::env::set_current_dir(Path::new(&path))?;
        Ok(())
    })?;

    // Function which returns the current executiong path as string
    let getcwd = lua.create_function(|_, ()| {
        let path = std::env::current_dir()?;
        Ok(path.to_string_lossy().to_string())
    })?;

    // Function to open a URL in the default Opener
    let open_link = lua.create_function(|_, url: String| {
        deprecated!(
            "dapi_os.open_link",
            "0.2.0",
            "The function changed its name, use 'dapi_os.open()' instead!"
        );

        open::that(url).map_err(|e| mlua::Error::external(format!("Cannot open URL: {}", e)))?;
        Ok(())
    })?;

    // Function to open a File in the default program or a Link
    let open = lua.create_function(|_, file: String| {
        open::that(file).map_err(|e| mlua::Error::external(format!("Cannot open: {}", e)))?;
        Ok(())
    })?;

    // Function to run a command in the commandline
    let run = lua.create_function(|lua, command: String| {
        deprecated!(
            "dapi_os.run",
            "0.1.10",
            "The function is although contained in the Lua STD or use 'dapi_os.run2()'"
        );

        #[cfg(target_os = "windows")]
        let output = Command::new("cmd").arg("/C").arg(&command).output()?;

        #[cfg(not(target_os = "windows"))]
        let output = Command::new("sh").arg("-c").arg(&command).output()?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let table = lua.create_table_from(vec![
            (
                "status",
                Value::Integer(output.status.code().unwrap_or(-1) as i64),
            ),
            ("stdout", Value::String(lua.create_string(&stdout)?)),
            ("stderr", Value::String(lua.create_string(&stderr)?)),
        ])?;

        Ok(table)
    })?;

    // Function to run a sync command in the Terminal
    let run2 = lua.create_function(|lua, command: String| {
        let mut child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(&command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?
        };

        let mut stdout = child.stdout.take().expect("stdout");
        let mut stderr = child.stderr.take().expect("stderr");

        let stdout_handle = thread::spawn(move || {
            let mut buffer = [0; 1024];
            let mut output = Vec::new();

            loop {
                match stdout.read(&mut buffer) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        // Sofort ausgeben, so wie terminal
                        let s = String::from_utf8_lossy(&buffer[..n]);
                        print!("{}", s);
                        output.extend_from_slice(&buffer[..n]);
                    }
                    Err(e) => {
                        eprintln!("Error reading stdout: {}", e);
                        break;
                    }
                }
            }

            String::from_utf8_lossy(&output).into_owned()
        });

        let stderr_handle = thread::spawn(move || {
            let mut buffer = [0; 1024];
            let mut output = Vec::new();

            loop {
                match stderr.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        let s = String::from_utf8_lossy(&buffer[..n]);
                        eprint!("{}", s);
                        output.extend_from_slice(&buffer[..n]);
                    }
                    Err(e) => {
                        eprintln!("Error reading stderr: {}", e);
                        break;
                    }
                }
            }

            String::from_utf8_lossy(&output).into_owned()
        });

        let status = child.wait()?;
        let stdout = stdout_handle.join().unwrap();
        let stderr = stderr_handle.join().unwrap();

        let table = lua.create_table_from(vec![
            ("status", Value::Integer(status.code().unwrap_or(-1) as i64)),
            ("stdout", Value::String(lua.create_string(&stdout)?)),
            ("stderr", Value::String(lua.create_string(&stderr)?)),
        ])?;

        Ok(table)
    })?;

    // TODO
    // Only the error is printed is async
    // Function to run a command Async
    let run3 = lua.create_function(|lua, command: String| {
        #[cfg(target_os = "windows")]
        let mut child = Command::new("cmd")
            .arg("/C")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        #[cfg(not(target_os = "windows"))]
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        // Stdout thread
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(line) = line {
                    println!("[stdout] {}", line);
                    // Optional: Lua-Callback aufrufen hier
                }
            }
        });

        // Stderr thread
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(line) = line {
                    eprintln!("[stderr] {}", line);
                    // Optional: Lua-Callback aufrufen hier
                }
            }
        });

        let status = child.wait()?; // Warten, bis Prozess beendet ist

        let table = lua.create_table_from(vec![(
            "status",
            Value::Integer(status.code().unwrap_or(-1) as i64),
        )])?;

        Ok(table)
    })?;

    // Binde split_path
    let split_fn = lua.create_function(|_, path: String| Ok(split_path(&path)))?;

    // Binde secure_path
    // TODO
    // Add Usage to the Docs
    //
    // local path1 = "foo/bar/baz"
    // local path2 = "../etc/passwd"
    //
    // local secure1 = secure_path(path1)
    // local secure2 = secure_path(path2)
    //
    // print("Secure1:", secure1)         --> foo/bar/baz
    // print("Secure2:", secure2)         --> nil (unsicher)
    //
    // if secure2 == nil then
    //     print("Path2 ist unsicher!")
    // end
    //
    let secure_fn = lua.create_function(|_, path: String| match secure_path(&path) {
        Ok(p) => Ok(Some(p.to_string_lossy().into_owned())),
        Err(_) => Ok(None),
    })?;

    // Binde join_path
    let join_fn = lua.create_function(|_, parts: mlua::Table| {
        // Lua Tabelle in Vec<String> umwandeln
        let mut vec = Vec::new();
        for pair in parts.sequence_values::<String>() {
            vec.push(pair?);
        }
        Ok(join_path(vec))
    })?;

    table.set("get_os_info", get_os_info)?;
    table.set("os", os)?;
    table.set("chdir", chdir)?;
    table.set("getcwd", getcwd)?;
    table.set("open_link", open_link)?;
    table.set("open", open)?;
    table.set("run", run)?;
    table.set("run2", run2)?;
    table.set("run3", run3)?;
    table.set("split_path", split_fn)?;
    table.set("secure_path", secure_fn)?;
    table.set("join_path", join_fn)?;

    Ok(table)
}

#[cfg(test)]
mod lua_tests {
    use crate::helper::dir::join_path;
    use mlua::Lua;

    #[test]
    fn test_lua_split_path() -> mlua::Result<()> {
        let lua = Lua::new();
        let split = lua.create_function(|_, path: String| Ok(super::split_path(&path)))?;
        lua.globals().set("split_path", split)?;

        let result: mlua::Table = lua.load(r#"return split_path("foo/bar/baz")"#).eval()?;
        let parts: Vec<String> = result.sequence_values().collect::<Result<_, _>>()?;

        assert_eq!(parts, vec!["foo", "bar", "baz"]);
        Ok(())
    }

    #[test]
    fn test_lua_join_path() -> mlua::Result<()> {
        let lua = Lua::new();
        let join = lua.create_function(|_, parts: Vec<String>| Ok(super::join_path(parts)))?;
        lua.globals().set("join_path", join)?;

        let path: String = lua
            .load(r#"return join_path({"foo", "bar", "baz"})"#)
            .eval()?;

        assert!(path.ends_with("foo/bar/baz") || path.ends_with("foo\\bar\\baz"));
        Ok(())
    }

    #[test]
    fn test_lua_secure_path_safe() -> mlua::Result<()> {
        let lua = Lua::new();
        let secure = lua.create_function(|_, path: String| Ok(super::secure_path(&path)))?;
        lua.globals().set("secure_path", secure)?;

        let is_safe: bool = lua.load(r#"return secure_path("some/safe/path")"#).eval()?;

        assert!(is_safe);
        Ok(())
    }

    #[test]
    fn test_lua_secure_path_unsafe() -> mlua::Result<()> {
        let lua = Lua::new();
        let secure = lua.create_function(|_, path: String| Ok(super::secure_path(&path)))?;
        lua.globals().set("secure_path", secure)?;

        let is_safe: bool = lua
            .load(r#"return secure_path("../../etc/passwd")"#)
            .eval()?;

        assert!(!is_safe);
        Ok(())
    }

    #[test]
    fn test_lua_combined_usage() -> mlua::Result<()> {
        let lua = Lua::new();

        // Bindings
        lua.globals().set(
            "split_path",
            lua.create_function(|_, path: String| Ok(super::split_path(&path)))?,
        )?;
        lua.globals().set(
            "join_path",
            lua.create_function(|_, parts: Vec<String>| Ok(super::join_path(parts)))?,
        )?;
        lua.globals().set(
            "secure_path",
            lua.create_function(|_, path: String| match super::secure_path(&path) {
                Ok(p) => Ok(Some(p.to_string_lossy().into_owned())),
                Err(_) => Ok(None),
            })?,
        )?;

        let (path, is_secure): (String, bool) = lua
            .load(
                r#"
            local parts = split_path("../../etc/passwd")
            local path = join_path(parts)
            local secure = secure_path(path) ~= nil
            return path, secure
            "#,
            )
            .eval()?;

        assert!(path.contains("etc") && path.find("passwd").is_some());
        assert!(!is_secure); // sicherheitsprüfung soll hier fehlschlagen
        Ok(())
    }

    #[test]
    fn test_join_path_basic() {
        let parts = vec!["home".to_string(), "user".to_string(), "docs".to_string()];
        let joined = join_path(parts);
        #[cfg(windows)]
        assert_eq!(joined, r"home\user\docs");
        #[cfg(not(windows))]
        assert_eq!(joined, "home/user/docs");
    }

    #[test]
    fn test_join_path_empty() {
        let parts: Vec<String> = vec![];
        let joined = join_path(parts);
        assert!(joined.is_empty());
    }

    #[test]
    fn test_join_path_single() {
        let parts = vec!["folder".to_string()];
        let joined = join_path(parts);
        assert_eq!(joined, "folder");
    }

    #[test]
    fn test_join_path_lua_binding() {
        let lua = Lua::new();

        // Binde join_path
        let join_fn = lua
            .create_function(|_, parts: mlua::Table| {
                let mut vec = Vec::new();
                for pair in parts.sequence_values::<String>() {
                    vec.push(pair?);
                }
                Ok(join_path(vec))
            })
            .unwrap();

        lua.globals().set("join_path", join_fn).unwrap();

        // Teste Lua-Seite mit gültiger Tabelle
        let result: String = lua
            .load(
                r#"
            return join_path({"home", "user", "docs"})
        "#,
            )
            .eval()
            .unwrap();

        #[cfg(windows)]
        assert_eq!(result, r"home\user\docs");
        #[cfg(not(windows))]
        assert_eq!(result, "home/user/docs");

        // Teste Lua-Seite mit leerer Tabelle
        let result_empty: String = lua
            .load(
                r#"
            return join_path({})
        "#,
            )
            .eval()
            .unwrap();

        assert_eq!(result_empty, "");

        // Teste Lua-Seite mit einzelnen Element
        let result_single: String = lua
            .load(
                r#"
            return join_path({"folder"})
        "#,
            )
            .eval()
            .unwrap();

        assert_eq!(result_single, "folder");
    }
}
