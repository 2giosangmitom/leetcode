package leetcode.ValidParentheses;

import java.util.ArrayList;

class Solution {
  public boolean isValid(String s) {
    int length = s.length();
    if (length % 2 != 0) {
      return false;
    }

    ArrayList<Character> stack = new ArrayList<Character>();
    for (int i = 0; i < length; i++) {
      if (s.charAt(i) == '(' || s.charAt(i) == '{' || s.charAt(i) == '[') {
        stack.add(s.charAt(i));
      } else {
        if (stack.isEmpty()) {
          return false;
        }

        if ((s.charAt(i) == ')' && stack.getLast() == '(')
            || (s.charAt(i) == '}' && stack.getLast() == '{')
            || (s.charAt(i) == ']' && stack.getLast() == '[')) {
          stack.removeLast();
        } else {
          return false;
        }
      }
    }

    return stack.isEmpty();
  }
}

