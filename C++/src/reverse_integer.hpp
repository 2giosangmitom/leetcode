#ifndef REVERSE_INTEGER_HPP
#define REVERSE_INTEGER_HPP

#include <climits>

class Solution {
public:
  int reverse(int x) {
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
  };
};

#endif // REVERSE_INTEGER_HPP