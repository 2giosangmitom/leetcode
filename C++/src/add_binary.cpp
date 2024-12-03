#include <add_binary.hpp>
using namespace std;

void trimLeadingZeros(string &s) {
  size_t firstOne = s.find('1');
  s = (firstOne == string::npos) ? "0" : s.substr(firstOne);
}

string Solution::addBinary(string a, string b) {
  trimLeadingZeros(a);
  trimLeadingZeros(b);

  int len_a = a.size();
  int len_b = b.size();

  if (len_a < len_b) {
    return addBinary(b, a);
  }

  int j = len_b - 1;
  int carry = 0;

  for (int i = len_a - 1; i >= 0; i--) {
    int bit1 = a[i] - '0';
    int sum = bit1 + carry;

    if (j >= 0) {
      int bit2 = b[j] - '0';
      sum += bit2;
      j--;
    }

    int bit = sum % 2;
    carry = sum / 2;

    a[i] = bit + '0';
  }

  if (carry > 0) {
    a = '1' + a;
  }

  return a;
}
