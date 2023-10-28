# @param {String} s
# @return {Integer}
def roman_to_int(s)
  roman_map = {
    I: 1,
    V: 5,
    X: 10,
    L: 50,
    C: 100,
    D: 500,
    M: 1000,
  }

  result = 0

  s.reverse.each_char do |c|
    number = roman_map[c.to_sym]
    return -1 if number == nil

    if number * 4 < result
      result -= number
    else
      result += number
    end
  end

  return result
end
