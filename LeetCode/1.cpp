// Two Sum
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    for (int i = 0; i < nums.size(); ++i) {
      int need = target - nums[i];
      for (int j = 0; j < nums.size(); ++j) {
        if (nums[j] == need && j != i) {
          return {i, j};
        }
      }
    }
    return {-1};
  }
};
