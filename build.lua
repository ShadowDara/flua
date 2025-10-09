-- Build Script for Luajit

-- Requirements Linux
-- - cargo
-- - python3
-- - pip

-- Requirements Windows
-- - cargo
-- - python
-- - NSIS

-- Imports
local dapi = require("dapi")
local dapi_io = require("dapi_io")
local dapi_os = require("dapi_os")

dapi.check_version("0.1.11", true)

-- Function to run the tests
function run_tests()
    local osdata2 = dapi_os.os()
    if osdata2.win then
        -- Tests for Windows
        print("Testing does not work automatically on windows yet!")
    else
        -- Tests for Linux / MacOS
        os.execute("./test/command_tester.sh")
        os.execute("target/debug/luajit test/main.lua")
        os.execute("target/debug/luajit test/data.lua")
        os.execute("target/debug/luajit test/http_async.lua")
    end
end

-- Function to build for windows
function build_windows()
    print("Running Build for Windows")
    os.execute("python build/win.py")
end

-- Function to build for Linux
function build_linux()
    print("Running Build for Linux")
    os.execute("cargo build --release")
end

-- Function to build for MacOS
function build_macos()
    print("Running Build for MacOS")
    os.execute("cargo build --release")
end

-- Start the Script
print("Luajit Build Script")

if arg and arg[1] == "test" then
    run_tests()
    os.exit(0)
end

-- Copies the Changelog from Repo Root to /docs/
dapi_io.copy_file("CHANGELOG.md", "docs/CHANGELOG.md")

print("Check Code")
os.execute("cargo check")

print("Format Code")
os.execute("cargo fmt")

-- Import get release Tags file
print("Get Release Tags")
local tags = require("build.get_tags")

-- Build the Documentation
print("Build the Documentation")
os.execute("pip install -r build/requirements.txt")
os.execute("mkdocs build")

local osdata = dapi_os.os()
if osdata.win then
    build_windows()
elseif osdata.lin then
    build_linux()
elseif osdata.mac then
    build_macos()
end

print("Finished Build")
