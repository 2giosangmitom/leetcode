// Missing Number
#include <vector>
using namespace std;

class Solution {
public:
  int missingNumber(vector<int> &nums) {
    int sum = 0;
    int total = nums.size() * (nums.size() + 1) / 2;
    for (int i = 0; i < nums.size(); i++) {
      sum += nums[i];
    }
    return total - sum;
  }
};
