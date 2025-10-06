dapi_os = require("dapi_os")
local cwd = dapi_os.getcwd()
print(cwd)

-- Imports
local dapi_json = require("dapi_json")
local dapi_toml = require("dapi_toml")
local dapi_dotenv = require("dapi_dotenv")
local dapi_yaml = require("dapi_yaml")
local dapi_ini  = require("dapi_ini")
local dapi_base64 = require("dapi_base64")

-- Helpers
local function assert_equal(a, b, msg)
  local function serialize(val)
    if type(val) == "table" then
      local str = "{"
      for k,v in pairs(val) do
        str = str .. tostring(k) .. "=" .. serialize(v) .. ","
      end
      return str .. "}"
    else
      return tostring(val)
    end
  end

  if type(a) == "table" and type(b) == "table" then
    for k,v in pairs(a) do
      assert_equal(v, b[k], msg .. " (key: " .. k .. ")")
    end
    for k,v in pairs(b) do
      assert_equal(v, a[k], msg .. " (key: " .. k .. ")")
    end
  else
    assert(a == b, msg .. ": expected " .. serialize(b) .. ", got " .. serialize(a))
  end
end

-- JSON Tests
local function test_json()
  print("Testing JSON...")
  local original = {
    name = "Lua",
    list = {1, 2, 3},
    active = true
  }

  local encoded = dapi_json.encode(original)
  local decoded = dapi_json.decode2(encoded)

  assert_equal(decoded.name, "Lua", "JSON name field mismatch")
  assert_equal(decoded.list[2], 2, "JSON list field mismatch")
  assert_equal(decoded.active, true, "JSON bool field mismatch")
end

-- TOML Tests
local function test_toml()
  print("Testing TOML...")
  local toml_data = [[
title = "Mein Beispiel"

[user]
name = "Alice"
age = 30
]]

  local parsed = dapi_toml.decode(toml_data)
  assert_equal(parsed.title, "Mein Beispiel", "TOML title mismatch")
  assert_equal(parsed.user.name, "Alice", "TOML user.name mismatch")
  assert_equal(parsed.user.age, 30, "TOML user.age mismatch")

  local re_encoded = dapi_toml.encode(parsed)
  assert(type(re_encoded) == "string", "TOML re-encoding failed")
end

-- DotENV Tests
local function test_dotenv()
  local env_file = "test/test.env"
  print("Testing DotENV...")
  dapi_dotenv.load(env_file)
  dapi_dotenv.set("TEST_KEY", "hello")
  local value = dapi_dotenv.get("TEST_KEY")
  assert_equal(value, "hello", "DotENV get/set mismatch")

  -- Optional: Load from file (".env" or "custom.env")
  -- pcall used in case file not present
  local success, err = pcall(function() dapi_dotenv.load(env_file) end)
  if not success then
    print("No .env file to load (skipping load test).")
  end
end

-- YAML Tests
local function test_yaml()
  print("Testing YAML...")
  local yaml_str = [[
name: ChatGPT
version: 4
features:
  - mlua
  - yaml
]]
  local data = dapi_yaml.decode(yaml_str)
  assert_equal(data.name, "ChatGPT", "YAML name mismatch")
  assert_equal(data.features[1], "mlua", "YAML array mismatch")

  local re_encoded = dapi_yaml.encode(data)
  assert(type(re_encoded) == "string", "YAML encoding failed")
end

-- INI Tests
local function test_ini()
  print("Testing INI...")
  local ini_str = [[
[general]
name = Max
active = true

[settings]
volume = 80
]]

  local parsed = dapi_ini.parse(ini_str)
  assert_equal(parsed.general.name, "Max", "INI general.name mismatch")
  assert_equal(parsed.settings.volume, "80", "INI settings.volume mismatch (note: always string)")

  local back_to_ini = dapi_ini.convert(parsed)
  assert(type(back_to_ini) == "string", "INI convert failed")
end

-- Base64 Tests
local function test_base64()
  print("Testing Base64...")

  local input = "Hallo, Welt!"
  local expected_encoded = "SGFsbG8sIFdlbHQh"

  local encoded = dapi_base64.encode(input)
  assert_equal(encoded, expected_encoded, "Base64 encoding mismatch")

  local decoded = dapi_base64.decode(encoded)
  assert_equal(decoded, input, "Base64 decoding mismatch")

  -- Edge case: empty string
  local empty_encoded = dapi_base64.encode("")
  assert_equal(empty_encoded, "", "Base64 empty encode failed")

  local empty_decoded = dapi_base64.decode("")
  assert_equal(empty_decoded, "", "Base64 empty decode failed")
end

-- Master runner
local function run_all_tests()
  print("Running all config format tests...\n")
  test_json()
  test_toml()
  test_dotenv()
  test_yaml()
  test_ini()
  test_base64()
  print("\nAll tests passed!")
end

run_all_tests()
