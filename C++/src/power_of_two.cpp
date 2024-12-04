#include <cmath>
#include <power_of_two.hpp>
using namespace std;

bool Solution::isPowerOfTwo(int n) {
  if (n <= 0) {
    return false;
  }

  if (n == 1) {
    return true;
  }

  if (n % 2 != 0) {
    return false;
  }

  double need = log2(n);
  double t = (int)need;
  return t == need;
}
