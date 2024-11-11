#include <two_sum.hpp>
#include <unordered_map>

std::vector<int> Solution::twoSum(std::vector<int> &nums, int target) {
  std::unordered_map<int, int> hashMap;
  for (int i = 0; i < (int)nums.size(); i++) {
    int remainder = target - nums[i];
    if (hashMap.find(remainder) != hashMap.end()) {
      return std::vector<int>{hashMap[remainder], i};
    }

    hashMap[nums[i]] = i;
  }
  return std::vector<int>{};
}