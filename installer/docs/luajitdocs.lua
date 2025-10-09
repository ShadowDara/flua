-- Requires Luajit by Shadowdara
-- https://github.com/ShadowDara/LuaAPI-Rust/releases/tag/v0.1.8

local dapi = require("dapi")
local dapi_os = require("dapi_os")
local dapi_http = require("dapi_http")

dapi.check_version("0.1.8", true)

print("[INFO] Running Luajit Docs")

dapi_os.open_link("http://127.0.0.1:4413")
local server = dapi_http.start_static_server("./docs", 4413)
