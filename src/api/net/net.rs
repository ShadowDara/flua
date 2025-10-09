use mlua::{Lua, Result};
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Function to fetch a website
    let fetch = lua.create_function(|_, url: String| {
        let client = Client::new();

        let response = client
            .get(&url)
            .header("User-Agent", "MyLuaRustApp/1.0") // <-- important
            .send();

        match response {
            Ok(resp) => match resp.text() {
                Ok(body) => Ok(body),
                Err(e) => Err(mlua::Error::external(format!(
                    "Fehler beim Lesen des Bodys: {}",
                    e
                ))),
            },
            Err(e) => Err(mlua::Error::external(format!("HTTP-Fehler: {}", e))),
        }
    })?;

    // Function to download a file
    let download_file = lua.create_function(|_, (url, destination): (String, String)| {
        match reqwest::blocking::get(&url) {
            Ok(mut resp) => {
                match File::create(&destination) {
                    Ok(mut out) => {
                        if copy(&mut resp, &mut out).is_ok() {
                            Ok(true) // Erfolgreich heruntergeladen
                        } else {
                            Ok(false) // Fehler beim Schreiben
                        }
                    }
                    Err(_) => Ok(false), // Fehler beim Datei erstellen
                }
            }
            Err(_) => Ok(false), // Fehler beim HTTP-GET
        }
    })?;

    table.set("fetch", fetch)?;
    table.set("download_file", download_file)?;

    Ok(table)
}
