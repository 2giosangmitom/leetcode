#include <binary_search.hpp>
#include <vector>
using namespace std;

int Solution::search(vector<int> &nums, int target) {
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
}