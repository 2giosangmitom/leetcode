#include <solution_tmpl.h>

int Solution::removeElement(vector<int> &nums, int val) {
  int k = 0;
  for (size_t i = 0; i < nums.size(); i++) {
    if (nums[i] != val) {
      nums[k] = nums[i];
      k++;
    }
  }
  return k;
}
