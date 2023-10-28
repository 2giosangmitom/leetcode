# frozen_string_literal: true

# @param {Integer} x
# @return {Integer}
def reverse(x)
  result = 0
  is_negative = x.negative?
  x = x.abs if is_negative

  while x != 0
    last_number = x % 10
    result = result * 10 + last_number

    return 0 if result > 2**31 - 1 || result < -2**31

    x = (x / 10).floor
  end

  is_negative ? -result : result
end
