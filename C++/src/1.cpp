#include <solution_tmpl.h>
#include <unordered_map>

vector<int> Solution::twoSum(vector<int> &nums, int target) {
  unordered_map<int, int> hashMap;
  for (int i = 0; i < nums.size(); i++) {
    int remainder = target - nums[i];
    if (hashMap.find(remainder) == nullptr) {
      hashMap.insert({nums[i], i});
    } else {
      return vector<int>{hashMap[remainder], i};
    }
  }
  return vector<int>{};
}
