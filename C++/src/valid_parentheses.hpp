#ifndef VALID_PARENTHESES_HPP
#define VALID_PARENTHESES_HPP

#include <string>
#include <vector>

class Solution {
public:
  bool isValid(std::string s) {
    int length = s.length();
    if (length % 2 != 0) {
      return false;
    }

    std::vector<char> stack;
    for (char c : s) {
      if (c == '(' || c == '{' || c == '[') {
        stack.push_back(c);
      } else {
        if (stack.empty()) {
          return false;
        }
        char top = stack.back();
        if ((c == ')' && top == '(') || (c == '}' && top == '{') ||
            (c == ']' && top == '[')) {
          stack.pop_back();
        } else {
          return false;
        }
      }
    }

    return stack.empty();
  };
};

#endif // VALID_PARENTHESES_HPP