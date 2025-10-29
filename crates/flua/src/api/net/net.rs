use mlua::{Error, Lua, Result};
use reqwest::Client;
use std::fs::File;
use std::io::copy;
use tokio::runtime::Runtime;

// TODO
// Improve this Code and make it very Performant

/// Globale Tokio-Runtime (nur einmal erzeugt)
static RUNTIME: once_cell::sync::Lazy<Runtime> =
    once_cell::sync::Lazy::new(|| Runtime::new().expect("Tokio runtime creation failed"));

pub fn register(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;

    // Fetch (asynchron ausgeführt)
    let fetch = lua.create_function(|_, url: String| {
        let result = RUNTIME.block_on(async {
            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .map_err(|e| Error::external(format!("Client error: {}", e)))?;

            let resp = client
                .get(&url)
                .header("User-Agent", "MyLuaRustApp/1.0")
                .send()
                .await
                .map_err(|e| Error::external(format!("HTTP error: {}", e)))?;

            if !resp.status().is_success() {
                return Err(Error::external(format!("HTTP status: {}", resp.status())));
            }

            let body = resp
                .text()
                .await
                .map_err(|e| Error::external(format!("Read body error: {}", e)))?;

            Ok(body)
        });
        result
    })?;

    // Download (asynchron ausgeführt)
    let download_file = lua.create_function(|_, (url, destination): (String, String)| {
        let result = RUNTIME.block_on(async {
            let client = Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .map_err(|e| Error::external(format!("Client error: {}", e)))?;

            let resp = client
                .get(&url)
                .send()
                .await
                .map_err(|e| Error::external(format!("HTTP error: {}", e)))?;

            if !resp.status().is_success() {
                return Err(Error::external(format!("HTTP status: {}", resp.status())));
            }

            let bytes = resp
                .bytes()
                .await
                .map_err(|e| Error::external(format!("Download read error: {}", e)))?;

            let mut out = File::create(&destination)
                .map_err(|e| Error::external(format!("Create file error: {}", e)))?;

            copy(&mut bytes.as_ref(), &mut out)
                .map_err(|e| Error::external(format!("Write error: {}", e)))?;

            Ok(true)
        });
        result
    })?;

    table.set("fetch", fetch)?;
    table.set("download_file", download_file)?;
    Ok(table)
}

#[cfg(test)]
mod tests {
    use super::*;
    use httptest::matchers::request;
    use httptest::{Expectation, Server, responders::*};
    use mlua::Function;
    use mlua::Lua;
    use std::fs;

    #[test]
    fn test_fetch_with_local_server() {
        let lua = Lua::new();
        let api = register(&lua).expect("Failed to register Lua functions");

        // Lokaler Testserver
        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/"))
                .respond_with(status_code(200).body(r#"{"url": "http://localhost/"}"#)),
        );

        let url = server.url("/").to_string();
        let fetch: Function = api.get("fetch").unwrap();
        let result: String = fetch.call(url).expect("Fetch failed");

        let json: serde_json::Value = serde_json::from_str(&result).expect("Invalid JSON");
        assert_eq!(json["url"], "http://localhost/");
    }

    #[test]
    fn test_download_file_with_local_server() {
        let lua = Lua::new();
        let api = register(&lua).expect("Failed to register Lua functions");

        let server = Server::run();
        server.expect(
            Expectation::matching(request::method_path("GET", "/file.txt"))
                .respond_with(status_code(200).body("Hello, world!")),
        );

        let url = server.url("/file.txt").to_string();
        let tmp_file = "test_download_local.txt";

        let download: Function = api.get("download_file").unwrap();
        let result: bool = download
            .call((url, tmp_file.to_string()))
            .expect("Download failed");

        assert!(result);
        let content = fs::read_to_string(tmp_file).expect("Failed to read file");
        assert_eq!(content, "Hello, world!");

        // Aufräumen
        let _ = fs::remove_file(tmp_file);
    }
}
