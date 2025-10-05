## import
to start, you need to import the api in your script, you can do it with
```lua
local dapi = require("dapi")
```

## version
a function which returns the current running version of luajit as a string

**Usage**
```lua
print(dapi.version())
```

## check_version
a function to check if the correct version of Luajit is used, first parameter
is the correct Version and second parameter is a Boolean for showing a warning
or info message

**Usage**
```lua
dapi.check_version("0.1.9", true)
```

## get_colors
a function which returns a Lua Table containing a lot of usable
ANSI Color Codes for colored output

**Usage**
```lua
local colors = dapi.get_colors()
print(colors.red + "Hallo" + colors.end)
```

**Different Color Codes**
**IMPORANT: Dont use the Color names in UPPERCASES, use lowercase letters instead !!!**
```rust
// Color codes for Colorful printing with Ansi Colorcodes
// Credit to for colorcodes
// https://ss64.com/nt/syntax-ansi.html
pub const END: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";

pub const ITALIC: &str = "\x1b[3m";
pub const UNDERLINED: &str = "\x1b[4m";

pub const REVERSE_TEXT: &str = "\x1b[7m";

pub const NOT_UNDERLINED: &str = "\x1b[24m";

pub const POSITIVE_TEXT: &str = "\x1b[27m";

pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE:  &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";

pub const BG_BLACK: &str = "\x1b[40m";
pub const BG_RED: &str = "\x1b[41m";
pub const BG_GREEN: &str = "\x1b[42m";
pub const BG_YELLOW: &str = "\x1b[43m";
pub const BG_BLUE:  &str = "\x1b[44m";
pub const BG_PURPLE: &str = "\x1b[45m";
pub const BG_CYAN: &str = "\x1b[46m";
pub const BG_WHITE: &str = "\x1b[47m";

pub const BRIGHT_BLACK: &str = "\x1b[90m";
pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_GREEN: &str = "\x1b[92m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
pub const BRIGHT_BLUE: &str = "\x1b[94m";
pub const BRIGHT_PURLPE: &str = "\x1b[95m";
pub const BRIGHT_CYAN: &str = "\x1b[96m";
pub const BRIGHT_WHITE: &str = "\x1b[97m";

pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
pub const BG_BRIGHT_RED: &str = "\x1b[101m";
pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
pub const BG_BRIGHT_PURLPE: &str = "\x1b[105m";
pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";
```
