# Daras Lua API

a Lua API written in Rust to make scripting much easier!

> [!TIP]
> You this for scripting prevent the need install one extra library for every new script you want to make

Run a script with
```sh
luajit <your-script.lua>
```

to suppress file and os access ad `-safe` at the end

Read the [CHANGELOG](./CHANGELOG.md) or the Docs for more Infos

**Feel free to open an Issue for a build request or feature request!**

> [!CAUTION]
> The Program is **NOT** guaranteed stable before **1.0.0**

## Running
For Infos or help run:
```sh
luajit --help
```

## Install

> [!IMPORTANT]
> Building, installing, and using the workflow on **Linux** (especially ARM64/aarch64) and **macOS** may currently require manual setup or additional dependencies.  
> If you're running into issues, you're **not alone** — improvements are planned, but help is **HIGHLY** appreciated.

### Linux

```sh
# Download the script
curl -L -o "LuajitInstall.sh" "https://raw.githubusercontent.com/ShadowDara/LuaAPI-Rust/refs/heads/main/installer/install.sh"

# Make it executable
chmod +x LuajitInstall.sh

# Execute the script
./LuajitInstall.sh "v0.1.11"
```

### Windows

Download the NSIS installer [here](https://github.com/ShadowDara/LuaAPI-Rust/releases)

> [!NOTE]
> Install Script for windows is although planned and hopefully coming soon!

## Feature Overview
### Data Parsing
- JSON
- Toml
- .env
- Yaml

## 1.0.0 Roadmap
- [ ] add full data parsing
- [ ] compabilllity with luarocks
- [ ] multiversion modules
- [ ] module creation like cargo
- [ ] Autoupdater / Version Installer / Selector
- [ ] Safe Mode
- [ ] multiversion compabillity and multiversion installer
- [ ] adding VS-Code support for the functions
- [ ] UI which can be used directly from lua
- [ ] ( ??? Windows API Bindings ??? )
- [ ] gh workflow and script to install Luajit + (Dockerfile)

<!-- ## Stats 0.1.11 -->
<!--

.\cloc . --md --out=cloc_report.md --exclude-dir=target

-->

## Build

```sh
cargo run build.lua
```

## Testing

```sh
cargo run test/main.lua
cargo run test/data.lua
```
