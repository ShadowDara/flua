use mlua::{Lua, Result};
use std::fs;
use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::path::Path;

use dirs_next::{
    audio_dir, cache_dir, config_dir, data_dir, data_local_dir, desktop_dir, document_dir,
    download_dir, home_dir, picture_dir, video_dir,
};

use crate::utils::zip_utils::{unzip_file, zip_dir};

use crate::deprecated;

use crate::helper::dir::copy_dir_recursive;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Function to ZIP a directory, do not use it yet!
    let zip = lua.create_function(|_, (src, dest): (String, String)| {
        deprecated!(
            "dapi_io.zip",
            "0.1.10",
            "The function could go horribly wrong, use at your own risk!"
        );
        zip_dir(&src, &dest).map_err(|e| mlua::Error::external(format!("Zip error: {}", e)))
    })?;

    // Function to unZIP a directory, do not use it yet!
    let unzip = lua.create_function(|_, (zip, dest): (String, String)| {
        deprecated!(
            "dapi_io.unzip",
            "0.1.10",
            "The function could go horribly wrong, use at your own risk!"
        );
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

    // Delete a directory recursively
    let delete_dir = lua.create_function(|_, dir: String| {
        fs::remove_dir_all(&dir)
            .map_err(|e| mlua::Error::external(format!("Delete dir error: {}", e)))
    })?;

    // Copy a file
    let copy_file = lua.create_function(|_, (from, to): (String, String)| {
        fs::copy(&from, &to)
            .map(|_| ()) // Ignore number of bytes copied
            .map_err(|e| mlua::Error::external(format!("Copy file error: {}", e)))
    })?;

    // Copy Dir
    let copy_dir = lua.create_function(|_, (from, to): (String, String)| {
        copy_dir_recursive(Path::new(&from), Path::new(&to))
            .map_err(|e| mlua::Error::external(format!("Copy directory error: {}", e)))?;
        Ok(())
    })?;

    // Create a file
    let create_file = lua.create_function(|_, file: String| {
        deprecated!(
            "dapi_io.create_file",
            "0.1.10",
            "The function is although contained in the Lua STD"
        );
        fs::File::create(&file)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Create file error: {}", e)))
    })?;

    // Write Data to a file
    let write_file = lua.create_function(|_, (file, content): (String, String)| {
        deprecated!(
            "dapi_io.write_file",
            "0.1.10",
            "The function is although contained in the Lua STD"
        );
        fs::write(&file, &content)
            .map(|_| ())
            .map_err(|e| mlua::Error::external(format!("Write file error: {}", e)))
    })?;

    // Funktion to read a file and return the content as a String
    let rf = lua.create_function(|_, path: String| match fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(e) => Err(mlua::Error::external(e)),
    })?;

    // Function to append data to the file
    let append_file = lua.create_function(|_, (file, content): (String, String)| {
        // Datei im Append-Modus öffnen (oder erstellen, wenn sie nicht existiert)
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file)
            .map_err(|e| mlua::Error::external(format!("Datei-Fehler: {}", e)))?;

        // Inhalt anhängen
        f.write_all(content.as_bytes())
            .map_err(|e| mlua::Error::external(format!("Schreib-Fehler: {}", e)))?;

        Ok(())
    })?;

    // Function get the content of a Folder as an Array
    let get_folder_content = lua.create_function(|lua_ctx, path: String| {
        let entries = fs::read_dir(&path).map_err(mlua::Error::external)?;

        let lua_table = lua_ctx.create_table()?; // neue Lua-Tabelle
        let mut index = 1;

        for entry in entries {
            let entry = entry.map_err(mlua::Error::external)?;
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            lua_table.set(index, file_name_str)?;
            index += 1;
        }

        Ok(lua_table)
    })?;

    // Function to get the size of an file
    let get_file_size = lua.create_function(|_, path: String| {
        fs::metadata(&path)
            .map(|metadata| metadata.len())
            .map_err(|e| mlua::Error::external(format!("Failed to get file size: {}", e)))
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
    table.set("delete_dir", delete_dir)?;
    table.set("copy_file", copy_file)?;
    table.set("copy_dir", copy_dir)?;
    table.set("create_file", create_file)?;
    table.set("write_file", write_file)?;
    table.set("rf", rf)?;
    table.set("append_file", append_file)?;
    table.set("get_folder_content", get_folder_content)?;
    table.set("get_file_size", get_file_size)?;
    table.set("read_line", read_line)?;

    Ok(table)
}
