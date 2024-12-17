#ifndef BINARY_SEARCH_HPP
#define BINARY_SEARCH_HPP

#include <vector>
using namespace std;

class Solution {
public:
  int search(vector<int> &nums, int target) {
    int start = 0;
    int end = nums.size() - 1;

    while (start <= end) {
      int middle = (end - start) / 2 + start;

      if (nums[middle] == target) {
        return middle;
      }

      if (nums[middle] < target) {
        start = middle + 1;
      } else {
        end = middle - 1;
      }
    }

    return -1;
  };
};

#endif // BINARY_SEARCH_HPP
