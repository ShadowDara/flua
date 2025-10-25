# Dapi HTTP

## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_http = require("dapi_http")
```

## start_static_server
function to start a static http server from a directory

**Usage**
```lua
dapi_http.start_static_server(".", 8080)
```
