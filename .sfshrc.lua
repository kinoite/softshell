--[[
*=========================================*
SOFTSHELL CONFIGURATION
you'd usually place this in your home directory.
you can rename this file to either:
        config.lua
        .config.lua
        .sfshrc
        .sfshrc.lua
        or .sfsh.rc
*=========================================*
]]--

history_file = "~/.sfsh_history"

local ANSI_RESET = "\x1b[0m"
local ANSI_BOLD = "\x1b[1m"
local ANSI_GREEN = "\x1b[32m"
local ANSI_BLUE = "\x1b[34m"
local ANSI_CYAN = "\x1b[36m"
local ANSI_WHITE = "\x1b[37m"

local Softhostname = require("softhostname")

function get_prompt()
    local user = os.getenv("USER") or "user"
    local host = Softhostname.get_hostname() or "host"
    local cwd_full = io.popen("pwd"):read("*l") or ""
    cwd_full = cwd_full:gsub("\n", "")

    local cwd_display = cwd_full:match("([^/\\]+)$") or "/"
    if cwd_display == "" then cwd_display = "/" end

    return string.format(
        "%s%s@%s%s:%s%s%s %s$%s ",
        ANSI_GREEN, user, host, ANSI_RESET,
        ANSI_BLUE, cwd_display, ANSI_RESET,
        ANSI_BOLD, ANSI_RESET
    )
end

mycommands = {}

function mycommands.hello(name)
    name = name or "World"
    print(string.format("%sHello, %s%s%s! Welcome to sfsh.%s", ANSI_CYAN, ANSI_BOLD, name, ANSI_RESET, ANSI_RESET))
end

print(string.format("%s--- sfsh: Simple Configuration Loaded ---%s", ANSI_BOLD, ANSI_RESET))
print("Type 'mycommands.hello()' to say hi!")
