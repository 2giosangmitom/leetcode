// Valid Parentheses
#include <iostream>
#include <stack>
using namespace std;

class Solution {
public:
  bool isValid(string s) {
    if (s.size() % 2 != 0) {
      return false;
    }
    stack<char> stack;
    for (char c : s) {
      if (c == '(' || c == '{' || c == '[') {
        stack.push(c);
      } else {
        if (stack.empty()) {
          return false;
        }
        if (c == ')' && stack.top() == '(') {
          stack.pop();
        } else if (c == '}' && stack.top() == '{') {
          stack.pop();
        } else if (c == ']' && stack.top() == '[') {
          stack.pop();
        } else {
          return false;
        }
      }
    }
    return stack.empty();
  }
};
