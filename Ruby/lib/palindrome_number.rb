# @param {Integer} x
# @return {Boolean}
def is_palindrome(x)
  return false if x < 0

  # @param {Integer} n
  # @return {Integer}
  reverse = ->(n) {
    result = 0

    while n != 0
      last_digit = n % 10
      result = result * 10 + last_digit
      n = (n / 10).floor
    end

    return result
  }

  return reverse.call(x) == x
end
