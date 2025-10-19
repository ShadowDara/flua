-- Script to package the Docs in a Zip Archive

local dapi = require("dapi")

-- ALWAYS CHECK !!!
dapi.check_version("0.2.0", true, true)

local dapi_os = require("dapi_os")
local dapi_io = require("dapi_io")

-- Build the Docs
dapi_os.run2("mkdocs build")

-- Zip the Docs
dapi_io.zip("site", "docs.zip")
