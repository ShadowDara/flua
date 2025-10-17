<!--
Testing Commands and Other Commands

cargo build && test/command_tester.sh


-->

# flua

a Lua API written in Rust to make scripting much easier!

> [!TIP]
> You this for scripting prevent the need install one extra library for every new script you want to make

Run a script with
```sh
flua <your-script.lua>
```

to suppress file and os access ad `-safe` at the end

Read the [CHANGELOG](./CHANGELOG.md) or the Docs for more Infos

**Feel free to open an Issue for a build request or feature request!**

> [!CAUTION]
> The Program is **NOT** guaranteed stable before **1.0.0**

## Running
For Infos or help run:
```sh
flua --help
```

## Install

> [!IMPORTANT]
> Building, installing, and using the workflow on **Linux** (especially ARM64/aarch64) and **macOS** may currently require manual setup or additional dependencies.  
> If you're running into issues, you're **not alone** — improvements are planned, but help is **HIGHLY** appreciated.
> Feel free to open an Issue to request a build for a specific plattform

### Windows

Download the NSIS installer for windows [here](https://github.com/ShadowDara/flua/releases)

> [!NOTE]
> Install Script for windows is although planned and hopefully coming soon!

### Linux

**Dependencies**
```sh
sudo apt update
sudo apt install build-essential
sudo apt install pkg-config libssl-dev
```

**Install**

```sh
# Download the script
curl -L -o "FluaInstall.sh" "https://raw.githubusercontent.com/ShadowDara/flua/refs/heads/main/installer/install.sh"

# Make it executable
chmod +x FluaInstall.sh

# Execute the script
./FluaInstall.sh "v0.1.13"
```

## Feature Overview

### Data Parsing
- JSON
- Toml
- .env
- Yaml
- INI
- Base64
- XML

## Roadmap

Feel free to open an Issue to add Ideas to the Roadmap which could
useful.

### 0.2.0
- [x] renaming to flua
- [ ] http server with custom api


### 0.3.0
- [ ] remove all deprecated functions or fix them
- [ ] multi version kompabillity
- [ ] working moduels
- [ ] Safe Mode
- [ ] gh workflow and script to install Luajit + (Dockerfile)

### 0.4.0
- [ ] multiversion modules
- [ ] module creation like cargo
- [ ] Autoupdater / Version Installer / Selector

### 0.5.0
- [ ] change modul names (only when multiversion compabillity works !)
- [ ] adding VS-Code support for the functions
- [ ] UI which can be used directly from lua

### 0.6.0
- [ ] ( ??? Windows API Bindings ??? )

### 1.0.0
- [ ] compabillity with LuaRocks
- [ ] builds for all sorts of systems

<!-- ## Stats 0.1.11 -->
<!--

.\cloc . --md --out=cloc_report.md --exclude-dir=target

-->

## Build

```sh
cargo run build.lua
```

## Testing

Testing commands with Shell
```sh
chmod +x ./test/command_tester.sh
./test/command_tester.sh
```

Testing Lua
```sh
cargo run test/main.lua
cargo run test/data.lua
cargo run test/http_async.lua
cargo run test/api_server.lua
```
