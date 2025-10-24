# Dapi HTTP Async

## import
```lua
local dapi_http_async = require("dapi_http_async")
```

## start_static_server
start a async http server which is running in the background

**Usage**
```lua
-- Directory, Port
dapi_http_async.start_static_server("public", 8080)
```

## stop_static_server
stop the async http server which running in the background, does not raises an error when no server is running on that port from luajit

**Usage**
```lua
-- Port
dapi_http_async.stop_static_server(8080)
```

# http api async
need to write docs
