use mlua::{Lua, Result, Value};
use std::io::{BufRead, BufReader};
use std::process::Command;
use std::process::Stdio;
use std::thread;

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
        std::env::set_current_dir(path)?;
        Ok(())
    })?;

    // Function which returns the current executiong path as string
    let getcwd = lua.create_function(|_, ()| {
        let path = std::env::current_dir()?;
        Ok(path.to_string_lossy().to_string())
    })?;

    // Function to open a URL in the default Opener
    let open_link = lua.create_function(|_, url: String| {
        open::that(url).map_err(|e| mlua::Error::external(format!("Cannot open URL: {}", e)))?;
        Ok(())
    })?;

    // Function to run a command in the commandline
    let run = lua.create_function(|lua, command: String| {
        deprecated!(
            "dapi_os.run",
            "0.1.10",
            "The function is although contained in the Lua STD"
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

    let run2 = lua.create_function(|lua, command: String| {
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

    table.set("get_os_info", get_os_info)?;
    table.set("os", os)?;
    table.set("chdir", chdir)?;
    table.set("getcwd", getcwd)?;
    table.set("open_link", open_link)?;
    table.set("run", run)?;
    table.set("run2", run2)?;

    Ok(table)
}
