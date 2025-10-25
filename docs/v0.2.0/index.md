# Version 0.2.0
- Release Date: 20.10.2025

## Download
- [v0.2.0](https://github.com/ShadowDara/LuaAPI-Rust/releases/tag/v0.2.0)

## Changelog
### Big Changes
The programm is now called `flua`

### Full Change List
- renamed programm to flua
- added flalaxy in another repository, the installer for flua -
https://github.com/ShadowDara/flalaxy
- added server with api but does not work :(
- added `dapi_os.run2()` - runs a command with instant no flush,
the command does not wait with the output until its finished
- added `open` to open files and links in the standard program
- changed some messages and changed error time to 3 seconds instead of 2
- reorganised Code and Help Message in the main.rs file
- extented the version check function to interupt the script if wanted,
and added an optional 3rd argument `(bool)` if the script should be interrupted
- added function to throw an custom Error
- added `startmenu` and `local_startmenu` from `some_default_dirs` to
`dapi_io.get_default_directories`
- added clear - a function to clear the Terminal Window Content
- added a function run3() to run a command async in the terminal
- added a global `SCRIPT_FULL_PATH` var to Lua
- added Functions in the OS Lib `secure_path()`, `split_path()` and `join_path()`
- added a Function to decode JSON with comments
- JSON encode function makes now always compressed JSON, but pretty JSON can
be enable with an 2nd Optional Boolean Argument as `true`
- fixed an error in the copy file function
- added Tests for Base64
- added `h` as a command to print the help message
- fixed Output bug in `run2()`
- added tests for the YAML Module
- added a BUILDS file and changed some stuff in the README.md
- added Workflow to zip the documentation and upload it to the next Github Workflow
- fixed some paths in the IO and the OS Library
- fixed Workflow for Windows Builds
- fixed ZIP functions and added Tests to them
- added `SCRIPT_DIRECTOY` as a global var to Lua
- added Tests for join paths and other paths function
- added a Makefile for the build so i dont have to remember commands
- added `dir` tests
- added function to convert compressed json in pretty json and the other way around
- added tests for JSON
- added Tests to the SCRIPT Vars in Lua
- fixed an Error in Join path function and a build script error
- fixed secure path for UNIX Systems
- added Commands to the Makefile
- added Module Stuff
- added Module Tests
- `run3()` runs now a command with colors

Read the DOCS for Version 0.2.0 for more Infos

## Info
Documentation is NOT complete yet, but all funktion headers are listed in the
documentation

## All Imports
```lua
local dapi = require("dapi")
local dapi_io = require("dapi_io")
local dapi_os = require("dapi_os")
local dapi_http = require("dapi_http")
local dapi_json = require("dapi_json")
local dapi_toml = require("dapi_toml")
local dapi_dotenv = require("dapi_dotenv")
local dapi_yaml = require("dapi_yaml")
local dapi_ini  = require("dapi_ini")
local dapi_base64 = require("dapi_base64")
local dapi_xml = require("dapi_xml")
local dapi_http_async = require("dapi_http_async")
local dapi_net = require("dapi_net")
local dapi_time = require("dapi_time")
local dapi_api_async = require("dapi_api_async")
```
