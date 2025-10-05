## JSON

### import
```lua
local dapi_json = require("dapi_json")
```

### decode2
a function to decode JSON to a LUa Table
```lua
local json_string = "{allo: "kkkk"}"
local json = dapi_json.decode2(json_string)
```

### encode
a function to encode a Lua Table to JSON
```lua
local lua_table = {
  key = "value",
  numbers = {1, 2, 3},
  flag = true,
}

local json_string = dapi_json.encode(lua_table)
```

## Toml

### import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_http = require("dapi_toml")
```

### decode
decode toml data to a Lua Table

```lua
local input = [[
title = "Mein Beispiel"

[user]
name = "Alice"
age = 30
]]

local result = dapi_toml.decode(input)

print(result.title)           --> "Mein Beispiel"
print(result.user.name)       --> "Alice"
print(result.user.age)        --> 30
```

### encode
encode a Lua table to toml data

```lua
local data = {
    title = "Konfiguration",
    settings = {
        width = 1920,
        height = 1080,
        fullscreen = true
    }
}

local toml_string = dapi_toml.encode(data)
print(toml_string)
```
