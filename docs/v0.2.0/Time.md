# Time

## import
```lua
dapi_time = require("dapi_time")
```

## stopwatch

**Example**
```lua
local stopwatch = require("stopwatch")

stopwatch.new_stopwatch("sw1")
stopwatch.start("sw1")

dapi.wait(3000)

local elapsed = stopwatch.read("sw1")
print("Elapsed time:", elapsed)

stopwatch.pause("sw1")
dapi.wait(1000)

local paused_time = stopwatch.read("sw1")
print("Should be same (paused):", paused_time)

stopwatch.start("sw1")
dapi.wait(1000)

print("Total after resume:", stopwatch.read("sw1"))

stopwatch.stop("sw1")
```

## waitforever
a simple wait function which waits until the programm is interrupted
with `CTRL + C` or closed

**Usage**
```lua
dapi_time.waitfr()
```
