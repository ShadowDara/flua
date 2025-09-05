use mlua::{Lua, Result};
use mlua::Table;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::copy;
use std::sync::Arc;
use mlua::Error as LuaError;
use mlua::{StdLib, LuaOptions};
use zip::{read::ZipArchive, ZipWriter};

use std::io::{self, Write};
use walkdir::WalkDir;
use zip::write::FileOptions;

// Execute an Lua Script
pub fn execute_script(file: &str, safe_mode: &bool) -> Result<()> {
    // Datei existiert?
    if !Path::new(file).exists() {
        eprintln!("Error: File '{}' not found!", file);
        return Ok(());
    }

    // Dateiinhalt lesen
    let script = fs::read_to_string(file)
        .map_err(|e| mlua::Error::external(format!("Error while reading: {}", e)))?;

    // Lua-VM erstellen
    let lua = Lua::new();
    if *safe_mode {
        println!("Will be implmented soon!");
        return Ok(());
        /*
        lua = Lua::new_with(
            StdLib::BASE | LuaStdLib::MATH | LuaStdLib::STRING | LuaStdLib::TABLE,
            LuaOptions::default(),
        )?;
        */
    }

    //
    //
    //
    // Registriere das API-Modul als Lua-Tabelle
    let dapi = lua.create_table()?;

    // Greet function
    let greet = lua.create_function(|_, name: String | {
        println!("Hello from Rust, {}!", name);
        Ok(())
    })?;

    // Calculation Function
    let add = lua.create_function(|_, (a, b): (i64, i64) | {
        Ok(a + b)
    })?;
    
    // Download Function
    // 1: Link -  2: Destination file
    let download = lua.create_function(|_, (url, destination): (String, String) | {
        let mut resp = match reqwest::blocking::get(url) {
            Ok(r) => r,
            Err(e) => return Err(LuaError::ExternalError(Arc::new(e))),
        };
        let mut out = File::create(destination)?;
        copy(&mut resp, &mut out)?;
        Ok(())
    })?;
    
    //TODO
    // Unzip Function
    // Zip FUnction
    // Open Link in default browser
    // Rename
    // Copy
    // Delete
    // Run Command
    // OS Infos

    // Register Lua functions
    dapi.set("greet", greet)?;
    dapi.set("add", add)?;
    dapi.set("download", download)?;
    
    // Register an Input Output API
    let dapi_io = lua.create_table()?;
    
    // Zip an Archive
    let zip = lua.create_function(|_, (src_dir, zip_path): (String, String)| {
        zip_dir(&src_dir, &zip_path)
            .map_err(|e| mlua::Error::external(format!("Zip error: {}", e)))
    })?;
    
    // Unzip a Zip Archive
    let unzip = lua.create_function(|_, (zip_path, dest_dir): (String, String)| {
        unzip_file(&zip_path, &dest_dir)
            .map_err(|e| mlua::Error::external(format!("Unzip error: {}", e)))
    })?;
    
    // Register IO Functions
    dapi_io.set("zip", zip)?;
    dapi_io.set("unzip", unzip)?;    

    // Modul beim Lua package.preload registrieren
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let preload: Table = package.get("preload")?;

    preload.set(
        "dapi",
        lua.create_function(move |_, ()| Ok(dapi.clone()))?,
    )?;
    
    preload.set(
        "dapi_io",
        lua.create_function(move |_, ()| Ok(dapi_io.clone()))?,
    )?;
    //
    //
    //

    // Skript ausführen
    lua.load(&script).exec()?;

    Ok(())
}

// UNzip file for Lua
fn unzip_file(zip_path: &str, dest_dir: &str) -> io::Result<()> {
    let file = fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    fs::create_dir_all(dest_dir)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = Path::new(dest_dir).join(file.name());

        if file.is_dir() {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(parent) = outpath.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}

// Zip a File for LUA
fn zip_dir(src_dir: &str, zip_path: &str) -> io::Result<()> {
    let file = fs::File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let walkdir = WalkDir::new(src_dir).into_iter();
    for entry in walkdir.filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(src_dir)).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = fs::File::open(path)?;
            io::copy(&mut f, &mut zip)?;
        } else if !name.as_os_str().is_empty() {
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    Ok(())
}

