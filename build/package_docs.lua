-- Script to package the Docs in a Zip Archive

local dapi_os = require("dapi_os")
local dapi_io = require("dapi_io")

-- Build the Docs
dapi_os.run2("mkdocs build")

-- Zip the Docs
dapi_io.zip("site", "docs.zip")
