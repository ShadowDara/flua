dapi_os = require("dapi_os")

print("Tests for flua")

local cwd = dapi_os.getcwd()

-- Print the Executing Directory
print(cwd)


dapi_os.run("echo hi")
dapi_os.run2("echo hi")
