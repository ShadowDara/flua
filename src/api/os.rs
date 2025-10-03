use mlua::{Lua, Result, Value};
use std::process::Command;
use sys_info::LinuxOSReleaseInfo;

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

        let os_type = sys_info::os_type().map_err(|e| mlua::Error::external(e.to_string()))?; // Einmal abfragen

        let windows = os_type == "Windows";
        let linux = os_type == "Linux";
        let macos = os_type == "Darwin";

        table.set("windows", windows)?;
        table.set("linux", linux)?;
        table.set("macos", macos)?;

        Ok(table)
    })?;

    // Function to open a URL in the default Opener
    let open_link = lua.create_function(|_, url: String| {
        open::that(url).map_err(|e| mlua::Error::external(format!("Cannot open URL: {}", e)))?;
        Ok(())
    })?;

    // Function to run a command in the commandline
    let run = lua.create_function(|lua, command: String| {
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

    table.set("get_os_info", get_os_info)?;
    table.set("os", os)?;
    table.set("open_link", open_link)?;
    table.set("run", run)?;

    Ok(table)
}
