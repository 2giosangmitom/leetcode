#ifndef REMOVE_ELEMENT_HPP
#define REMOVE_ELEMENT_HPP

#include <vector>

class Solution {
public:
  int removeElement(std::vector<int> &nums, int val) {
    int k = 0;
    for (size_t i = 0; i < nums.size(); i++) {
      if (nums[i] != val) {
        nums[k] = nums[i];
        k++;
      }
    }
    return k;
  };
};

#endif // REMOVE_ELEMENT_HPP