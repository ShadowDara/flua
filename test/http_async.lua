-- Imports
local dapi = require("dapi")
local dapi_http_async = require("dapi_http_async")

dapi.check_version("0.1.13", true)

print("Start Server")

-- Server asynchron starten
dapi_http_async.start_static_server("public", 8080)

dapi.wait(10000)

print("End Server")

-- später...
dapi_http_async.stop_static_server(8080)
dapi_http_async.stop_static_server(8080)
