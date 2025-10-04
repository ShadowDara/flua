## import
```lua
local dapi_json = require("dapi_json")
```

## json_decode2
a function to decode JSON to a LUa Table
```lua
local json_string = "{allo: "kkkk"}"
local json = dapi_json.decode2(json_string)
```

## json_encode
a function to encode a Lua Table to JSON
```lua
local lua_table = {
  key = "value",
  numbers = {1, 2, 3},
  flag = true,
}

local json_string = dapi_json.encode(lua_table)
```
