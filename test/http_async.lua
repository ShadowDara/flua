-- Imports
local dapi = require("dapi")
local dapi_http_async = require("dapi_http_async")
local dapi_time = require("dapi_time")

dapi.check_version("0.1.13", true)

print("Start Server")

-- Server asynchron starten
dapi_http_async.start_static_server("public", 8080)

print("Waiting 3 seconds")
dapi.wait(3000)

print("End Server")

-- später...
dapi_http_async.stop_static_server(8080)
dapi_http_async.stop_static_server(8080)


print("Test Stopwatch")

local stopwatch = require("dapi_time")

stopwatch.new_stopwatch("sw1")
stopwatch.start("sw1")

dapi.wait(3000)

local elapsed = stopwatch.read("sw1")
print("Elapsed time:", elapsed)

stopwatch.pause("sw1")
dapi.wait(1000)

local paused_time = stopwatch.read("sw1")
print("Should be same (paused):", paused_time)

stopwatch.start("sw1")
dapi.wait(1000)

print("Total after resume:", stopwatch.read("sw1"))

stopwatch.stop("sw1")
