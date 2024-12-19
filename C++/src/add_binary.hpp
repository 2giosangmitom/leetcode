#ifndef ADD_BINARY_HPP
#define ADD_BINARY_HPP

#include <algorithm>
#include <string>

using namespace std;

class Solution {
public:
  string addBinary(string &a, string &b) {
    int len_a = a.size();
    int len_b = b.size();

    int i = len_a - 1;
    int j = len_b - 1;
    int carry = 0;

    string result;
    result.reserve(max(len_a, len_b) + 1);

    while (i >= 0 || j >= 0 || carry > 0) {
      int bit1 = (i >= 0) ? a[i--] - '0' : 0;
      int bit2 = (j >= 0) ? b[j--] - '0' : 0;

      int sum = bit1 + bit2 + carry;
      result.push_back((sum % 2) + '0');
      carry = sum / 2;
    }

    reverse(result.begin(), result.end());
    return result;
  }
};

#endif // ADD_BINARY_HPP
