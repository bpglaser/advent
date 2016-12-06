local md5 = require 'md5'

local input = arg[1]
assert(input, "no puzzle input passed")

function stringify(password)
    local s = ""
    for i = 0, 7 do
        if password[i] == nil then
            s = s .. "_"
        else
            s = s .. password[i]
        end
    end
    return s
end

function full(password)
    for i = 0, 7 do
        if password[i] == nil then
            return false
        end
    end
    return true
end

local i = 0
local password = {}

while not full(password) do
    local tohash = input .. i
    local sum = md5.sumhexa(tohash)

    if string.sub(sum, 1, 5) == "00000" then
        local passwordindex = tonumber(string.sub(sum, 6, 6))
        if passwordindex ~= nil and password[passwordindex] == nil then
            password[passwordindex] = string.sub(sum, 7, 7)
            print("found: " .. tohash .. "\t=>\t" .. sum .. "\t[" .. stringify(password) .. "]")
        end
    end

    if i % 50000 == 0 then
        print("progress: " .. tohash)
    end

    i = i + 1
end

print("final: " .. stringify(password))
