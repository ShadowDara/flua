-- Windows Build for the Release Workflow

-- Run from the root of the gh repo !!!

local dapi = require("dapi")

-- ALWAYS CHECK !!!
dapi.check_version("0.2.0", true, true)

local dapi_os = require("dapi_os")

dapi_os.run2("mkdocs build")

dapi_os.run2("cargo build --release --target=x86_64-pc-windows-msvc")

dapi_os.run2("makensis installer/nsis/installer.nsi")

os.rename("target/x86_64-pc-windows-msvc/release/flua.exe", "flua.exe")
