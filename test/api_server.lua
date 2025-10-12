-- Imports
local web = require("dapi_api_async")
local time = require("dapi_time")
local dos = require("dapi_os")

-- API-Server auf Port 8081 starten
web.start_api_server(8081)
-- web.stop_api_server(8081)

-- web.start_api_server(8081, {
--   hello = function()
--     return {
--       message = "Hello from Lua!"
--     }
--   end
-- })

web.start_api_server(8082, {
  hello = "function() return { message = 'Hello from Lua string!' } end",
  now   = "return { now = os.date('%Y-%m-%d %H:%M:%S') }"
})

dos.open_link("http://127.0.0.1:8082/api/hello")

web.start_api_server(8083, {
    hello = function() return { message = "Hi" } end,      -- echte Funktion
    now = "os.date()",                                     -- Ausdruck
    greet = [[function() return { msg = "Hello!" } end]],  -- Mehrzeilige Function als String
    empty = nil                                           -- nil handler -> gibt nil zurück
})

time.waitfr()
