#include <add_digits.hpp>

// You can read this article: https://en.wikipedia.org/wiki/Digital_root
int Solution::addDigits(int num) {
  if (num == 0) {
    return 0;
  }
  return 1 + ((num - 1) % 9);
}