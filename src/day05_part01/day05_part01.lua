local md5 = require 'md5'

local input = arg[1]
assert(input, "no puzzle input passed")

local i = 0
local password = ""

while string.len(password) <= 8 do
    local tohash = input .. i
    local sum = md5.sumhexa(tohash)

    if string.sub(sum, 1, 5) == "00000" then
        password = password .. string.sub(sum, 6, 6)
        print("found: " .. tohash .. "\t=>\t" .. sum .. "\t[" .. password .. "]")
    end

    if i % 50000 == 0 then
        print("progress: " .. tohash)
    end

    i = i + 1
end

print("final: " .. password)
