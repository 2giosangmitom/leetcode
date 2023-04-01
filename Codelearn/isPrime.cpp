// https://codelearn.io/learning/thuat-toan-can-ban/3831
#include <iostream>
using namespace std;

bool isPrime(int n) {
  if (n < 2) {
    return false;
  }
  if (n == 2) {
    return true;
  }
  if (n % 2 == 0) {
    return false;
  }
  for (int i = 3; i < n / 2; i += 2) {
    if (n % i == 0) {
      return false;
    }
  }
  return true;
}

int main() {
  cout << "n = ";
  int n;
  cin >> n;
  if (isPrime(n)) {
    cout << n << " is prime";
  } else {
    cout << n << " is not prime";
  }
}
