#include <gtest/gtest.h>

#include <map>
#include <vector>
using namespace std;

class Solution {
 public:
  vector<int> twoSum(vector<int> &nums, int target) {
    map<int, int> map;
    for (int i = 0; i < nums.size(); i++) {
      int complement = target - nums[i];
      if (map.find(complement) != map.end()) {
        return {map[complement], i};
      }
      map[nums[i]] = i;
    }
    return {-1};
  }

  vector<int> twoSum2(vector<int> &nums, int target) {
    for (int i = 0; i < nums.size(); i++) {
      int complement = target - nums[i];
      for (int j = i + 1; j < nums.size(); j++) {
        if (nums[j] == complement) {
          return {i, j};
        }
      }
    }
    return {-1};
  }
};

// Tests
struct tt {
  vector<int> nums;
  int target;
  vector<int> want;
};

const tt cases[] = {
    {.nums = vector<int>{2, 7, 11, 15}, .target = 9, .want = vector<int>{0, 1}},
    {.nums = vector<int>{3, 2, 4}, .target = 6, .want = vector<int>{1, 2}},
    {.nums = vector<int>{3, 3}, .target = 6, .want = vector<int>{0, 1}},
    {.nums = vector<int>{1, 3, 4, 1, 25, 8}, .target = 30, .want = vector<int>{-1}},
};

TEST(twoSum, test) {
  Solution s;
  for (tt c : cases) {
    vector<int> result = s.twoSum(c.nums, c.target);
    vector<int> result2 = s.twoSum2(c.nums, c.target);
    ASSERT_EQ(result, c.want);
    ASSERT_EQ(result2, c.want);
  }
}
