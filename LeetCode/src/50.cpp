// Pow(x, n)
#include <iostream>
using namespace std;

class Solution {
public:
  double myPow(double x, int n) {
    double pow = n;
    if (n == 0) {
      return 1;
    }
    if (n < 0) {
      pow = -1.0 * n;
    }
    double result = n % 2 == 0 ? myPow(x * x, pow / 2) : myPow(x * x, (pow - 1) / 2) * x;
    return n < 0 ? 1 / result : result;
  }
};
