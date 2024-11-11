#include <climits>
#include <reverse_integer.hpp>

int Solution::reverse(int x) {
  long result = 0;

  while (x != 0) {
    int lastDigit = x % 10;
    result = result * 10 + lastDigit;
    if (result > INT_MAX || result < INT_MIN) {
      return 0;
    }
    x /= 10;
  }

  return result;
}