use mlua::{Lua, Result};
use std::fs;
use std::io::{self, BufRead};

use crate::utils::zip_utils::{zip_dir, unzip_file};

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    let zip = lua.create_function(|_, (src, dest): (String, String)| {
        zip_dir(&src, &dest).map_err(|e| mlua::Error::external(format!("Zip error: {}", e)))
    })?;

    let unzip = lua.create_function(|_, (zip, dest): (String, String)| {
        unzip_file(&zip, &dest).map_err(|e| mlua::Error::external(format!("Unzip error: {}", e)))
    })?;

    let create_dir = lua.create_function(|_, dir: String| {
        fs::create_dir_all(&dir)
            .map_err(|e| mlua::Error::external(format!("Create dir error: {}", e)))
    })?;

    let create_file = lua.create_function(|_, file: String| {
        fs::File::create(&file)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Create file error: {}", e)))
    })?;

    let write_file = lua.create_function(|_, (file, content): (String, String)| {
        fs::write(&file, &content)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Write file error: {}", e)))
    })?;

    let read_line = lua.create_function(|lua, (file, max_lines): (String, Option<usize>)| {
        let file = fs::File::open(&file)
            .map_err(|e| mlua::Error::external(format!("Open file error: {}", e)))?;
        let reader = io::BufReader::new(file);
        let lua_table = lua.create_table()?;

        for (i, line_result) in reader.lines().enumerate() {
            if let Some(max) = max_lines {
                if i >= max {
                    break;
                }
            }
            let line = line_result
                .map_err(|e| mlua::Error::external(format!("Read line error: {}", e)))?;
            lua_table.set(i + 1, line)?;
        }

        Ok(lua_table)
    })?;

    table.set("zip", zip)?;
    table.set("unzip", unzip)?;
    table.set("create_dir", create_dir)?;
    table.set("create_file", create_file)?;
    table.set("write_file", write_file)?;
    table.set("read_line", read_line)?;

    Ok(table)
}
