## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi_io = require("dapi_io")
```

## input
a simple function which can print a Message and then return the typed valued (same as the
`input()` function in Python)

**Usage**
```lua
local text = dapi_io.input("What is your Input?")
```

## zip

## unzip

## get_dafault_directories
a function which returns a lua table containing a lot of dafault directories

**Usage**
```lua
local dir = dapi_io.get_default_directories()
print(dir.home)
```

**Path Overview**
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
        <tr>
            <td>startmenu</td>
            <td></td>
            <td></td>
        </tr>
        <tr>
            <td>local startmenu</td>
            <td></td>
            <td></td>
        </tr>
    </tbody>
</table>

## create_dir
function to create a directory recursivly

**Usage**
```lua
dapi_io.create_dir("Wtf")
```

## delete_dir
function to delete a directory recursivly

**Usage**
```lua
dapi_io.delete_dir("Wtf")
```

## copy_file
function to copy a file

**Usage**
```lua
dapi_io.copy_file("hallo.txt", "wtf/hallo.txt")
```

## copy_dir
function to copy a directory from one place to another place

**Usage**
```lua
dapi_io.copy_dir("/dir", "/dir2")
```

## create_file

## write_file

## append_file
function to add data to an existing file

```lua
dapi_io.append_file("/tmp/test.txt", "Zeile 1\n")
dapi_io.append_file("/tmp/test.txt", "Zeile 2\n")
```

## get_file_size
a function to get the size of an file

**Usage**
```lua
dapi_os.write_file("wtf.txt")
local size = dapi_os.get_file_size("wtf.txt")
print(size)
```

## `read_line()`
Reads a text file and returns its contents as a Lua table, line by line. Optionally, a maximum number of lines can be specified.

**Lua Function Signature**

```lua
lines = read_line(path [, max_lines])
```

**Parameters**

| Name         | Type     | Required | Description                                           |
|--------------|----------|----------|-------------------------------------------------------|
| `path`       | `string` | Yes   | The file path to read from                            |
| `max_lines`  | `number` | No    | Maximum number of lines to read (optional)            |

**Return Value**

Returns a Lua `table` where each line of the file is stored as a string:

| Index | Value      |
|-------|------------|
| `1`   | First line |
| `2`   | Second line|

**Example (in Lua)**

```lua
local lines = read_line("example.txt", 5)

for i, line in ipairs(lines) do
    print(i .. ": " .. line)
end
```

**Error Handling**

- If the file cannot be opened, an error is raised:  
  `"Open file error: <details>"`
- If a line cannot be read, an error is raised:  
  `"Read line error: <details>"`
- File reading stops early if `max_lines` is provided and reached.
