// Pow(x, n)
#include <cmath>
#include <iostream>
using namespace std;

class Solution {
public:
  double myPow(double x, int n) {
    if (n == 0) {
      return 1;
    }
    int pow = abs(n);
    double result;
    if (pow % 2 == 0) {
      result = myPow(x * x, pow / 2);
    } else {
      result = myPow(x * x, (pow - 1) / 2) * x;
    }
    return n < 0 ? 1 / result : result;
  }
};
