## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_io = require("dapi_io")
```

## get_dafault_directories
a function which returns a lua table containing a lot of dafault directories

### Example Usage
```lua
local dir = dapi_io.get_default_directories()
print(dir.home)
```

### Path Overview
<table>
    <thead>
        <tr>
            <th>Name</th>
            <th>Windows</th>
            <th>Linux</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td>home</td>
            <td>C:\Users\username</td>
            <td>/home/username</td>
        </tr>
        <tr>
            <td>desktop</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>documents</td>
            <td></td>
            <td>~/Documents</td>
        </tr>
        <tr>
            <td>downloads</td>
            <td></td>
            <td>~/Downloads</td>
        </tr>
        <tr>
            <td>music</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>videos</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>pictures</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>config</td>
            <td>%APPDATA%</td>
            <td>~/.config</td>
        </tr>
        <tr>
            <td>data</td>
            <td>%APPDATA%</td>
            <td>~/.local/share</td>
        </tr>
        <tr>
            <td>localdata</td>
            <td>%LOCALAPPDATA%</td>
            <td>~/.local/share</td>
        </tr>
        <tr>
            <td>cache</td>
            <td>C:\Users\username\AppData\Local\Cache</td>
            <td>~/.cache</td>
        </tr>
    </tbody>
</table>
