#ifndef TWO_SUM_HPP
#define TWO_SUM_HPP

#include <unordered_map>
#include <vector>

class Solution {
public:
  std::vector<int> twoSum(std::vector<int> &nums, int target) {
    std::unordered_map<int, int> hashMap;
    for (int i = 0; i < (int)nums.size(); i++) {
      int remainder = target - nums[i];
      if (hashMap.find(remainder) != hashMap.end()) {
        return std::vector<int>{hashMap[remainder], i};
      }

      hashMap[nums[i]] = i;
    }
    return std::vector<int>{};
  };
};

#endif // TWO_SUM_HPP