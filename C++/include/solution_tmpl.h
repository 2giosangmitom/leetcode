#ifndef SOLUTION_TEMPLATE_H
#define SOLUTION_TEMPLATE_H

#include <string>
#include <vector>

class Solution {
public:
  std::vector<int> twoSum(std::vector<int> &nums, int target);
  int reverse(int x);
  bool isPalindrome(int x);
  std::string longestCommonPrefix(std::vector<std::string> &strs);
  bool isValid(std::string s);
};

#endif
