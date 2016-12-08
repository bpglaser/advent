lines = File.open("../../input/day06.txt").read().lines().each { |line| line.strip! }
result = ""
(0..lines[0].length - 1).each { |column|
    totals = Hash.new(0)
    lines.each { |line|
        totals[line[column]] += 1
    }
    result += totals.to_a().sort_by { |obj| obj[1] }[0][0]
}
puts result
