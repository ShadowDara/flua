-- Import the API
local dapi = require("dapi")
local dapi_os = require("dapi_os")
local dapi_io = require("dapi_io")

-- TEST Dapi
dapi.greet("ne")

print("Download Start")
local result = dapi.download("https://release-assets.githubusercontent.com/github-production-release-asset/1048529985/b1d7e156-382a-4345-a702-dfbf74361be4", "luajit.exe")
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
-- dapi_os.open_link("https://github.com/ShadowDara/LuaAPI-Rust/releases/")

print("\n---\n")

-- Command RUN
local result = dapi_os.run("echo Hello World")
print("Exit code:", result.status)
print("STDOUT:", result.stdout)
print("STDERR:", result.stderr)

-- Create Directory
dapi_io.create_dir("test_dir")
dapi_io.create_file("test_dir/test_file.exe")
dapi_io.write_file("test_dir/test_file.exe", "Hello World!\nThis is a test file.")


local lines = dapi_io.read_line("README.md", 5)  -- Nur 5 Zeilen
for i, line in ipairs(lines) do
    print(i, line)
end
