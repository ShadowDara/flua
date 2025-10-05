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

## DotENV

### get
Gets the value of an environment variable.

**Lua Usage**
```lua
local value = env.get("DATABASE_URL")
if value then
  print("Found:", value)
else
  print("Not set")
end
```

**Returns**
- `string`: the value if found
- `nil`: if the variable is not set

### load
Loads environment variables from a `.env` file into the process environment.

**Lua Usage**
```lua
env.load()             -- loads from ".env" by default
env.load("custom.env") -- loads from a custom file
```

**Errors**
Returns a Lua error if the file could not be found or parsed.

### set
Sets an environment variable (unsafe in multi-threaded contexts).

This uses `std::env::set_var`, which is `unsafe` as of Rust 1.77.
Only use this in single-threaded scenarios.

**Lua Usage**
```lua
env.set("MY_VAR", "123")
print(env.get("MY_VAR")) --> "123"
```

**Safety**
This function uses an `unsafe` block because modifying environment variables
is not thread-safe across all platforms.

**Errors**
Returns a Lua error if key or value contain null bytes (`\0`), which are invalid.
