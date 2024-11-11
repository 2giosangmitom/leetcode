#include <remove_element.hpp>

int Solution::removeElement(std::vector<int> &nums, int val) {
  int k = 0;
  for (size_t i = 0; i < nums.size(); i++) {
    if (nums[i] != val) {
      nums[k] = nums[i];
      k++;
    }
  }
  return k;
}