## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_os = require("dapi_os")
```

## get_os_info

## os
to function to check if the current used os is windows, linux or macos
and depending on that, run special Code

### Usage
```lua
local osdata = dapi_os.os()

if osdata.win then
    print("You are using Windows")
elseif osdata.lin then
    print("You are using Linux")
elseif osdata.mac then
    print("You are using MacOS")
end
```

## open_link

## run
