# CHANGELOG

## Newest Develepment
- added Tests for TOML
- added Tests for XML
- added Workflow to run tests on PUSH and PR
- added Tests for dotenv
- added Tests for INI Module
- added Tests for NET Module
- added Config File in `config_dir()/@shadowdara/flua/config.lua`
- loading the Config File
- added config file Infos to the help message
- added Tests for Time
- added Config Options
- added Input functions like `input()` in Python
- added SQLite support
- the wait time at the end can now be interrupted by pressing enter
- fixed net module for windows

## 0.2.0

### Big Changes
The programm is now called `flua`

### Full Change List
- renamed programm to flua
- added flalaxy in another repository, the installer for flua -
https://github.com/ShadowDara/flalaxy
- added server with api but does not work :(
- added `dapi_os.run2()` - runs a command with instant no flush,
the command does not wait with the output until its finished
- added `open` to open files and links in the standard program
- changed some messages and changed error time to 3 seconds instead of 2
- reorganised Code and Help Message in the main.rs file
- extented the version check function to interupt the script if wanted,
and added an optional 3rd argument `(bool)` if the script should be interrupted
- added function to throw an custom Error
- added `startmenu` and `local_startmenu` from `some_default_dirs` to
`dapi_io.get_default_directories`
- added clear - a function to clear the Terminal Window Content
- added a function run3() to run a command async in the terminal
- added a global `SCRIPT_FULL_PATH` var to Lua
- added Functions in the OS Lib `secure_path()`, `split_path()` and `join_path()`
- added a Function to decode JSON with comments
- JSON encode function makes now always compressed JSON, but pretty JSON can
be enable with an 2nd Optional Boolean Argument as `true`
- fixed an error in the copy file function
- added Tests for Base64
- added `h` as a command to print the help message
- fixed Output bug in `run2()`
- added tests for the YAML Module
- added a BUILDS file and changed some stuff in the README.md
- added Workflow to zip the documentation and upload it to the next Github Workflow
- fixed some paths in the IO and the OS Library
- fixed Workflow for Windows Builds
- fixed ZIP functions and added Tests to them
- added `SCRIPT_DIRECTOY` as a global var to Lua
- added Tests for join paths and other paths function
- added a Makefile for the build so i dont have to remember commands
- added `dir` tests
- added function to convert compressed json in pretty json and the other way around
- added tests for JSON
- added Tests to the SCRIPT Vars in Lua
- fixed an Error in Join path function and a build script error
- fixed secure path for UNIX Systems
- added Commands to the Makefile
- added Module Stuff
- added Module Tests
- `run3()` runs now a command with colors

Read the DOCS for Version 0.2.0 for more Infos

## 0.1.13 - 10.10.2025
- add auto publish for the mkdocs documentation
- restructered some Code
- added async http servers
- made the whole code async via Tokio
- added Version parser for modules
- added command testing script
- added option to get the Version
- added Exit Code 1 when an Error happends
- added better Build for Linux
- fixed dev container dependescies
- added NET Library
- added Script to get all versions
- reformatted some folders
- reformatted some scripts
- added option to give arguments to luajit
- added Client to the Download File function
- added Colors to the help Info
- added Docs to the NSIS Installer for Windows
- added NSIS Installer to Release Workflow
- added Docs publish Workflow
- added Install Workflow for Luajit
- added async stopwatch to stop time

Read the DOCS for Version 0.1.13 for more Infos

## 0.1.12 - 08.10.2025
- added yaml parser
- updated install script
- delered NSIS Admin Installer
- added a function to wait for a certain period of time
- added Ini Parser
- added Data Tests
- added Base64 Encoding
- added a function to copy a dir recursivly
- added a function to get the file size
- added Desktop Shortcut to the NSIS Installer
- added XML Parser
- added better help Message

Read the DOCS for Version 0.1.12 for more Infos

## 0.1.11 - 05.10.2025
- added Function to delete a directory
- added default script option to the Installer
- added Copy File Function
- add Built for Linux aarch 64
- added dotenv parsing
- added Download Shell Script

Read the DOCS for Version 0.1.11 for more Infos

## 0.1.10 - 05.10.2025
- added Function to Check the OS Type to make code in the script which runs only
on a particular OS
- added Function to get the default dirs
- added getcwd and chdir function
- added JSON Encoding and Decoding
- added TOML Parsing

Read the DOCS for Version 0.1.10 for more Infos

## 0.1.9 - 04.10.2025
- made the deprecated warning yellow
- reorganisized some code
- added ANSI Colorcodes
- added Lua Function to get the colorcodes
- started Docs
- updated check version message
- add NSIS Installer for windows

## 0.1.8 - 01.10.2025
- Read Files Line by Line
- Restructering the whole Code
- added return Value to `dapi.download` function
- added Version check
- add static HTTP Server which is not async yet which means the Server blocks the
script! Server can be stopped by just pressing enter
- open with now works on windows

## 0.1.7 - 07.09.2025
- added Function to create a directory
- to create files
- to write to files
- added help output

## 0.1.6 - 05.09.2025
- added OS and Version Info
- added Lua Test File
- added Icon for Windows

## 0.1.5 - 05.09.2025
- added Zipping and Unzipping Features

## 0.1.4 - 04.09.2025

## 0.1.1

## 0.1.0 - 02.09.2025
Intential Release
