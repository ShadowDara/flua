-- Imports
local dapi = require("dapi")
local dapi_io = require("dapi_io")
local dapi_json = require("dapi_json")
local dapi_net = require("dapi_net")

dapi.check_version("0.1.13", true)

-- Funktion, um alle Tags von einem GitHub Repository-Release zu bekommen
function get_github_release_tags(owner, repo)
    local url = "https://api.github.com/repos/" .. owner .. "/" .. repo .. "/releases"
    
    -- Fetch JSON-Daten von GitHub
    local jsondata = dapi_net.fetch(url)
    if not jsondata then
        print("Fehler beim Abrufen der Daten.")
        return nil
    end

    -- Parse JSON-Daten
    local releases = dapi_json.decode2(jsondata)
    if not releases then
        print("Fehler beim Parsen der JSON-Daten.")
        return nil
    end

    -- Extrahiere alle Tags
    local tags = {}
    for i, release in ipairs(releases) do
        if release.tag_name then
            table.insert(tags, release.tag_name)
        end
    end

    return tags
end

-- Beispielaufruf:
local tags = get_github_release_tags("shadowdara", "LuaAPI-Rust")

-- Ensure that the file exists
dapi_io.create_file("tags.txt")

-- Ausgabe der Tags
if tags then
    for i, tag in ipairs(tags) do
        print("Tag " .. i .. ": " .. tag)
        dapi_io.append_file("tags.txt", tag)
        dapi_io.append_file("tags.txt", "\n")
    end
end

-- Now sort the versions
