#include <algorithm>
#include <gtest/gtest.h>
#include <solution_tmpl.h>
#include <vector>

class RemoveElement : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(RemoveElement, case1) {
  vector<int> nums = {3, 2, 2, 3};
  vector<int> want = {2, 2};
  int val = 3;

  int result = solution.removeElement(nums, val);
  ASSERT_EQ(result, 2);
  ASSERT_EQ(nums.size(), 4);
  nums.pop_back();
  nums.pop_back();
  sort(nums.begin(), nums.end());
  ASSERT_EQ(nums, want);
}

TEST_F(RemoveElement, case2) {
  vector<int> nums = {0, 1, 2, 2, 3, 0, 4, 2};
  vector<int> want = {0, 0, 1, 3, 4};
  int val = 2;

  int result = solution.removeElement(nums, val);
  ASSERT_EQ(result, 5);
  nums.pop_back();
  nums.pop_back();
  nums.pop_back();
  sort(nums.begin(), nums.end());
  ASSERT_EQ(nums.size(), want.size());
}
