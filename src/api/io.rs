use mlua::{Lua, Result};
use std::fs;
use std::io::{self, BufRead};

use dirs_next::{
    home_dir, desktop_dir, document_dir, download_dir, 
    audio_dir, video_dir, picture_dir, config_dir, data_dir,
    data_local_dir, cache_dir
};

use crate::utils::zip_utils::{zip_dir, unzip_file};

use crate::deprecated;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Function to ZIP a directory, do not use it yet!
    let zip = lua.create_function(|_, (src, dest): (String, String)| {
        deprecated!("dapi_io.zip", "0.1.10", "The function could go horribly wrong, use at your own risk!");
        zip_dir(&src, &dest).map_err(|e| mlua::Error::external(format!("Zip error: {}", e)))
    })?;

    // Function to unZIP a directory, do not use it yet!
    let unzip = lua.create_function(|_, (zip, dest): (String, String)| {
        deprecated!("dapi_io.unzip", "0.1.10", "The function could go horribly wrong, use at your own risk!");
        unzip_file(&zip, &dest).map_err(|e| mlua::Error::external(format!("Unzip error: {}", e)))
    })?;

    // Functions to get the default directories, returns a Lua Tale
    let get_default_directories = lua.create_function(|lua, ()| {
        let table = lua.create_table()?;

        table.set("home", home_dir())?;
        table.set("desktop", desktop_dir())?;
        table.set("documents", document_dir())?;
        table.set("downloads", download_dir())?;
        table.set("music", audio_dir())?;
        table.set("videos", video_dir())?;
        table.set("pictures", picture_dir())?;

        table.set("config", config_dir())?;
        table.set("data", data_dir())?;
        table.set("localdata", data_local_dir())?;
        table.set("cache", cache_dir())?;

        Ok(table)
    })?;

    // Create a directory
    let create_dir = lua.create_function(|_, dir: String| {
        fs::create_dir_all(&dir)
            .map_err(|e| mlua::Error::external(format!("Create dir error: {}", e)))
    })?;

    // Create a file
    let create_file = lua.create_function(|_, file: String| {
        deprecated!("dapi_io.create_file", "0.1.10", "The function is although contained in the Lua STD");
        fs::File::create(&file)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Create file error: {}", e)))
    })?;

    // Write Data to a file
    let write_file = lua.create_function(|_, (file, content): (String, String)| {
        deprecated!("dapi_io.write_file", "0.1.10", "The function is although contained in the Lua STD");
        fs::write(&file, &content)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Write file error: {}", e)))
    })?;

    // Function to read a file line by line
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
    table.set("get_default_directories", get_default_directories)?;
    table.set("create_dir", create_dir)?;
    table.set("create_file", create_file)?;
    table.set("write_file", write_file)?;
    table.set("read_line", read_line)?;

    Ok(table)
}
