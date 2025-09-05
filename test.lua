-- Import the API
local dapi = require("dapi")
local dapi_os = require("dapi_os")

-- TEST Dapi
dapi.greet("ne")

local version = dapi.version()
print(version)

-- TEST Dapi_OS
local info = dapi_os.get_os_info()
print("OS Type:", info.os_type)
print("Release:", info.os_release)
print("Hostname:", info.hostname)
print("CPU cores:", info.cpu_num)
print("Total RAM (KB):", info.mem_total)

dapi_os.open_link("https://www.google.com")

