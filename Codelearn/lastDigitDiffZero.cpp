// https://codelearn.io/learning/thuat-toan-can-ban/1151
#include <iostream>
using namespace std;

int lastDigitDiffZero(int n) {
  long long result = 1;
  for (int i = 2; i <= n; i++) {
    result *= i;
    while (result % 10 == 0) {
      result /= 10;
    }
    result = result % 100;
  }
  while (result % 10 == 0) {
    result /= 10;
  }
  return result % 10;
}

int main() {
  cout << "n = ";
  int n;
  cin >> n;
  cout << lastDigitDiffZero(n);
}
