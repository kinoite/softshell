--[[
  softhostname v0.0.1
  a simple softshell module that fetches the machine's hostname
]]--

local M = {}

function M.get_hostname()
    local host = os.getenv("HOSTNAME") or os.getenv("COMPUTERNAME")
    if host then
        return host
    end

    local cmd_output
    if package.config:sub(1,1) == "\\" then
        cmd_output = io.popen("hostname"):read("*l")
    else
        cmd_output = io.popen("hostname"):read("*l")
    end

    if cmd_output then
        return cmd_output:gsub("\n", "")
    end

    return "localhost"
end

return M
