#ifndef SOLUTION_TEMPLATE_H
#define SOLUTION_TEMPLATE_H
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target);
  int reverse(int x);
  bool isPalindrome(int x);
  string longestCommonPrefix(vector<string> &strs);
};

#endif
