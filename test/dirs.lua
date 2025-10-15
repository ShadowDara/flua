local dapi_io = require("dapi_io")

-- Beispieltest für get_default_directories über dapi_io

local dirs = dapi_io.get_default_directories()

-- Prüfen, ob der Rückgabewert ein Table ist
assert(type(dirs) == "table", "Expected a table from get_default_directories")

-- Liste der erwarteten Schlüssel
local expected_keys = {
    "home", "desktop", "documents", "downloads",
    "music", "videos", "pictures",
    "config", "data", "localdata", "cache",
    "startmenu", "local_startmenu"
}

-- Alle Schlüssel überprüfen
for _, key in ipairs(expected_keys) do
    local value = dirs[key]
    assert(type(value) == "string" or value == nil, "Expected string or nil for key: " .. key)
    print(string.format("✓ %s = %s", key, value or "nil"))
end
