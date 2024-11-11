#include <algorithm>
#include <single_number.hpp>
#include <vector>

int Solution::singleNumber(std::vector<int> &nums) {
  int result = 0;
  for (size_t i = 0; i < nums.size(); i++) {
    result = result ^ nums[i];
  }
  return result;
}

int Solution::singleNumber2(std::vector<int> &nums) {
  std::sort(nums.begin(), nums.end());
  for (size_t i = 1; i < nums.size(); i += 2) {
    if (nums[i] != nums[i - 1]) {
      return nums[i - 1];
    }
  }
  return nums[nums.size() - 1];
}