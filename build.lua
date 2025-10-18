-- Build Script for Luajit

-- Requirements Linux
-- - cargo
-- - python3
-- - pip

-- Requirements Windows
-- - cargo
-- - python
-- - NSIS

-- Requirements Workflow

-- Imports
local dapi = require("dapi")
local dapi_io = require("dapi_io")
local dapi_os = require("dapi_os")

dapi.check_version("0.2.0", true)

-- Function to run the tests
local function run_tests()
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
    print("Finished Tests")
end

-- Function to build for windows
local function build_windows()
    print("Running Build for Windows")
    os.execute("python build/win.py")
end

-- Function to build for Linux
local function build_linux()
    print("Running Build for Linux")
    os.execute("cargo build --release")
end

-- Function to build for MacOS
local function build_macos()
    print("Running Build for MacOS")
    os.execute("cargo build --release")
end

-- Start the Script
print("Luajit Build Script")

-- Use the args for selecting now
if arg and arg[1] == "test" then
    -- Running the tests
    print("Running Tests")
    run_tests()

    -- Exit after running the tests
    os.exit(0)
elseif arg and arg[1] == "workflow" then
    -- Build Code for the Windows Workflow
    print("Build for Windows Workflow")

    dapi_os.run2("cargo build --release --target=x86_64-pc-windows-msvc")
    --dapi_os.run2("cargo build --release --target=aarch64-pc-windows-msvc")

    -- Finish after the Workflow build
    os.exit(0)
else
    -- Normal Local USE
    print("Run for normal local use")
end

-- Copies the Changelog from Repo Root to /docs/
dapi_io.copy_file("CHANGELOG.md", "docs/CHANGELOG.md")

print("Check Code")
os.execute("cargo check")

print("Format Code")
os.execute("cargo fmt")

-- Build the Documentation
print("Build the Documentation")
os.execute("pip install -r build/requirements.txt")
os.execute("mkdocs build")
local file = io.open("site/.nojekyll", "w")

if file then
    -- Text in die Datei schreiben
    file:write("# Ignore for Jekyll\n")

    -- Datei schließen
    file:close()
else
    print("Error while opening the file!")
end

-- Add Zipping for the documentation

local osdata = dapi_os.os()
if osdata.win then
    build_windows()
elseif osdata.lin then
    build_linux()
elseif osdata.mac then
    build_macos()
end

print("Finished Build")
