# Version 0.2.1
- Release Date: XX.XX.XXXX

## Download
- [v0.2.1](https://github.com/ShadowDara/LuaAPI-Rust/releases/tag/v0.2.1)

## Changelog
- added Tests for TOML
- added Tests for XML
- added Workflow to run tests on PUSH and PR
- added Tests for dotenv
- added Tests for INI Module
- added Tests for NET Module
- added Config File in `config_dir()/@shadowdara/flua/config.lua`
- loading the Config File

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

## ConfigFile

### Configscript
```lua
-- Create a new Config Value
config = {}

-- Set the default wait time
config.wait_time = 0
```
