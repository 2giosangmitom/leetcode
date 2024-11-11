#include <gtest/gtest.h>
#include <two_sum.hpp>

class TwoSumTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(TwoSumTest, increasing_order) {
  std::vector<int> nums = {2, 7, 11, 15};
  int target = 9;
  std::vector<int> want = {0, 1};

  std::vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, different_order) {
  std::vector<int> nums = {3, 2, 4};
  int target = 6;
  std::vector<int> want = {1, 2};

  std::vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, same_numbers) {
  std::vector<int> nums = {3, 3};
  int target = 6;
  std::vector<int> want = {0, 1};

  std::vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, no_solution) {
  std::vector<int> nums = {1, 2, 3};
  int target = 10;
  std::vector<int> want = {};

  std::vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}

TEST_F(TwoSumTest, empty_vector) {
  std::vector<int> nums = {};
  int target = 10;
  std::vector<int> want = {};

  std::vector<int> actual = solution.twoSum(nums, target);
  EXPECT_EQ(actual, want);
}
