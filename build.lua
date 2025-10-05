local dapi = require("dapi")
local dapi_io = require("dapi_io")
local dapi_os = require("dapi_os")

dapi.check_version("0.1.11", true)

function build_windows()
    print("Running Build for Windows")
end

function build_linux()
    print("Running Build for Linux")
end

function build_macos()
    print("Running Build for MacOS")
end

-- Start the Script
print("Luajit Build Script")

-- Copies the Changelog from Repo Root to /docs/
dapi_io.copy_file("CHANGELOG.md", "docs/CHANGELOG.md")

local osdata = dapi_os.os()
if osdata.win then
    build_windows()
elseif osdata.lin then
    build_linux()
elseif osdata.mac then
    build_macos()
end
