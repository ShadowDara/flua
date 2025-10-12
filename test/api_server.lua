local web = require("dapi_api_async")  -- Je nach Bindungssystem
local time = require("dapi_time")
local dos = require("dapi_os")

-- API-Server auf Port 8081 starten
web.start_api_server(8081)
web.stop_api_server(8081)

-- web.start_api_server(8081, {
--   hello = function()
--     return {
--       message = "Hello from Lua!"
--     }
--   end
-- })

dos.open_link("http://127.0.0.1/api/hello")

time.waitfr()
