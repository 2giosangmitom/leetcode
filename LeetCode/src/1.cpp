// Two Sum
#include <map>
#include <vector>
using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    map<int, int> hash;
    for (int i = 0; i < nums.size(); ++i) {
      int need = target - nums[i];
      if (hash.find(need) != hash.end()) {
        return {hash[need], i};
      }
      hash[nums[i]] = i;
    }
    return {-1};
  }
};