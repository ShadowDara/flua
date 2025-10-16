## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_os = require("dapi_os")
```

## `get_os_info()`

This Lua function returns basic system information such as OS type, version, hostname, CPU count, and total memory.

**Usage**

```lua
table = dapi_os.get_os_info()
```

**Return Value**

A Lua `table` with the following fields:

| Key         | Type     | Description                                               |
|-------------|----------|-----------------------------------------------------------|
| `os_type`   | `string` | The operating system type (e.g., `"Linux"`, `"Windows"`)  |
| `os_release`| `string` | OS release/version string                                 |
| `hostname`  | `string` | The system's hostname                                     |
| `cpu_num`   | `number` | Number of available CPU cores                             |
| `mem_total` | `number` | Total physical memory in kilobytes (KB)                   |

**Usage Example (in Lua)**

```lua
local info = get_os_info()

print("OS Type:   " .. info.os_type)
print("OS Version:" .. info.os_release)
print("Hostname:  " .. info.hostname)
print("CPU Cores: " .. info.cpu_num)
print("RAM Total: " .. info.mem_total .. " KB")
```

**Error Handling**

- If any system information is unavailable (e.g., due to permission issues), fallback values are used:
  - `"Unknown"` for strings
  - `0` for numbers
- This ensures the function always returns a valid Lua table without runtime errors.

## `os()`
to function to check if the current used os is windows, linux or macos
and depending on that, run special Code

**Usage**
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

## `chdir()`
a function to change the current execution directory

**Usage**
```lua
-- create a new directory
dapi_io.create_dir("wtf")

dapi_os.chdir("wtf")
```

## `getcwd()`
a function which returns the current executing directory as a string

**Usage**
```lua
local cwd = dapi_os.getcwd()
```

## `open_link()`
opens a link in the standard browser of the User

**Usage**
```lua
dapi_os.open_link("https://github.com/shadowdara")
```

## `open`
function to open a file or a link or etc in the standard programm
of the users computer

**Usage**
```lua
dapi_os.open("https://github.com/shadowdara/flua")
dapi_os.open("sample.pdf")
```

## `run()`

## `run2()`
runs a command with instant no flush,
the command does not wait with the output until its finished

## `run3()`

## `split_path()`

## `secure_path()`

## `join_path()`
