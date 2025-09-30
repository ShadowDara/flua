local dapi = require("dapi")
local dapi_http = require("dapi_http")

dapi.check_version("0.1.8", true)

print("Starting HTTP Server")
local server = dapi_http.start_static_server(".", 8080)
print("Server started")
