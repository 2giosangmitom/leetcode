#include <gtest/gtest.h>
#include <solution_tmpl.h>

Solution solution;

TEST(TwoSumTest, case1) {
  vector<int> nums = {2, 7, 11, 15};
  int target = 9;
  vector<int> want = {0, 1};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST(TwoSumTest, case2) {
  vector<int> nums = {3, 2, 4};
  int target = 6;
  vector<int> want = {1, 2};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST(TwoSumTest, case3) {
  vector<int> nums = {3, 3};
  int target = 6;
  vector<int> want = {0, 1};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}