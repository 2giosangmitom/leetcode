#include <add_two_numbers.hpp>
#include <gtest/gtest.h>

class AddTwoNumbersTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(AddTwoNumbersTest, case1) {
  auto l1 = ListNode::from({2, 4, 3});
  auto l2 = ListNode::from({5, 6, 4});

  auto actual = solution.addTwoNumbers(l1.get(), l2.get());
  auto want = ListNode::from({7, 0, 8});
  EXPECT_TRUE(*actual == want.get());
}

TEST_F(AddTwoNumbersTest, case2) {
  auto l1 = ListNode::from({0});
  auto l2 = ListNode::from({0});

  auto actual = solution.addTwoNumbers(l1.get(), l2.get());
  auto want = ListNode::from({0});
  EXPECT_TRUE(*actual == want.get());
}

TEST_F(AddTwoNumbersTest, case3) {
  auto l1 = ListNode::from({9, 9, 9, 9, 9, 9, 9});
  auto l2 = ListNode::from({9, 9, 9, 9});

  auto actual = solution.addTwoNumbers(l1.get(), l2.get());
  auto want = ListNode::from({8, 9, 9, 9, 0, 0, 0, 1});
  EXPECT_TRUE(*actual == want.get());
}
