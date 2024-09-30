#include <gtest/gtest.h>
#include <solution_tmpl.h>
using namespace std;

class TwoSumTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(TwoSumTest, FindsIndicesOfTwoSum) {
  vector<int> nums = {2, 7, 11, 15};
  int target = 9;
  vector<int> want = {0, 1};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, FindsIndicesInDifferentOrder) {
  vector<int> nums = {3, 2, 4};
  int target = 6;
  vector<int> want = {1, 2};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, HandlesDuplicateValues) {
  vector<int> nums = {3, 3};
  int target = 6;
  vector<int> want = {0, 1};

  vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}
