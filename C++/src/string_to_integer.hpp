#ifndef STRING_TO_INTEGER_HPP
#define STRING_TO_INTEGER_HPP

#include <climits>
#include <string>
using namespace std;

class Solution {
public:
  int myAtoi(string s) {
    int i = 0, n = s.length();
    // Skip leading white spaces
    while (i < n && s[i] == ' ') {
      i++;
    }

    if (i == n)
      return 0; // If string contains only spaces

    int sign = 1; // Default is positive
    if (s[i] == '+' || s[i] == '-') {
      sign = (s[i] == '-') ? -1 : 1;
      i++;
    }

    long long result = 0;
    while (i < n && isdigit(s[i])) {
      result = result * 10 + (s[i] - '0');
      if (result * sign > INT_MAX)
        return INT_MAX;
      if (result * sign < INT_MIN)
        return INT_MIN;
      i++;
    }

    return (int)(result * sign);
  };
};

#endif // STRING_TO_INTEGER_HPP
