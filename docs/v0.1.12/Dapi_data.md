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
```lua
local dapi_dotenv = require("dapi_dotenv")
```

### get
Gets the value of an environment variable.

**Lua Usage**
```lua
local value = dapi_dotenv.get("DATABASE_URL")
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
dapi_dotenv.load()             -- loads from ".env" by default
dapi_dotenv.load("custom.env") -- loads from a custom file
```

**Errors**
Returns a Lua error if the file could not be found or parsed.

### set
Sets an environment variable (unsafe in multi-threaded contexts).

This uses `std::env::set_var`, which is `unsafe` as of Rust 1.77.
Only use this in single-threaded scenarios.

**Lua Usage**
```lua
dapi_dotenv.set("MY_VAR", "123")
print(dapi_dotenv.get("MY_VAR")) --> "123"
```

**Safety**
This function uses an `unsafe` block because modifying environment variables
is not thread-safe across all platforms.

**Errors**
Returns a Lua error if key or value contain null bytes (`\0`), which are invalid.

## Yaml

### import
```lua
local dapi_yaml = require("dapi_yaml")
```

### `yaml.encode` and `yaml.decode`

Lua bindings for serializing and deserializing YAML using Rust + `mlua`.

**Lua Usage**

```lua
local data = yaml.decode([[
name: ChatGPT
version: 4
features:
  - mlua
  - yaml
]])

print(data.name)           --> "ChatGPT"
print(data.features[1])    --> "mlua"

local yaml_str = yaml.encode(data)
print(yaml_str)
```

**Functions**

`yaml.decode(yaml_str: string) → table`

Parses a YAML string and returns a Lua table.

`yaml.encode(table: table) → string`

Serializes a Lua table into a YAML string.

**Error Handling**

- Errors are thrown as Lua exceptions if parsing or serialization fails.
- Invalid Lua types (e.g. userdata, functions) cannot be encoded.

## INI

This API enables conversion between **INI files** and **Lua tables**. It is exposed through a `register(lua)` function that provides two key functions: `parse` and `convert`.

### import
```lua
local dapi_ini = require("dapi_ini")
```

### `parse(ini_string: string) -> table`

**Description:**  
Parses a string in INI format and returns a Lua table representing the structure and data.

**Parameters:**

- `ini_string` *(string)*: The contents of an INI file as a string.

**Returns:**

- *(table)*: A Lua table containing sections and key-value pairs from the INI file.

**Example:**

```lua
local ini = [[
[general]
name = Max
active = true

[settings]
volume = 80
]]

local parsed = dapi_ini.parse(ini)

-- Access values:
print(parsed.general.name)      --> "Max"
print(parsed.settings.volume)   --> "80"
```

### convert
This function takes a Lua Table and returns an INI File as a String

**Usage**
```lua
-- Using Value from above
local ini_b = dapi_ini.convert(parsed)
print(ini_b)
```

## Base64

### import
```lua
dapi_base64 = require("dapi_base64")
```

## encode
encodes a Text string to base 64

## decode
decodes a base64 string back to text

**Usage**
```lua
local dapi_base64 = require("dapi_base64")

local encoded = dapi_base64.encode("Hello, Lua!")
print(encoded) -- "SGVsbG8sIEx1YSE="

local decoded = dapi_base64.decode(encoded)
print(decoded) -- "Hello, Lua!"
```