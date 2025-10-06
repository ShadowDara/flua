dapi_os = require("dapi_os")

print("Tests for Luajit")

local cwd = dapi_os.getcwd()

-- Print the Executing Directory
print(cwd)
