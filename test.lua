-- Import the API
local dapi = require("dapi")
local dapi_os = require("dapi_os")

-- TEST Dapi
dapi.greet("ne")

local result = dapi.download("https://github.com/ShadowDara/LuaAPI-Rust/releases/download/v0.1.6/luajit-windows-x86_64.exe", "luajit.exe")
print(result)

local version = dapi.version()
print(version)

print("\n---\n")

-- TEST Dapi_OS
local info = dapi_os.get_os_info()
print("OS Type:", info.os_type)
print("Release:", info.os_release)
print("Hostname:", info.hostname)
print("CPU cores:", info.cpu_num)
print("Total RAM (KB):", info.mem_total)

-- Download
dapi_os.open_link("https://github.com/ShadowDara/LuaAPI-Rust/releases/")

print("\n---\n")

-- Command RUN
local result = dapi_os.run("echo Hello World")
print("Exit code:", result.status)
print("STDOUT:", result.stdout)
print("STDERR:", result.stderr)

