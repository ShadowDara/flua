local web = require("dapi_api_async")  -- Je nach Bindungssystem

-- API-Server auf Port 8081 starten
web.start_api_server(8081)

web.start_api_server(8081, {
  hello = function()
    return { message = "Hello from Lua!" }
  end
})
