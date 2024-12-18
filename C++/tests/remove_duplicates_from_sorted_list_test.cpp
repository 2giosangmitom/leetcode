#include <gtest/gtest.h>
#include <memory>
#include <remove_duplicates_from_sorted_list.hpp>
#include <vector>

class RemoveDuplicatesFromSortedListTest : public ::testing::Test {
protected:
  Solution solution;

  void check_equal(ListNode *actual, ListNode *expected) {
    EXPECT_TRUE(actual != nullptr ? *actual == expected : expected == nullptr);
  }
};

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesDuplicates) {
  auto head = ListNode::from({1, 1, 2});
  auto expected = ListNode::from({1, 2});
  auto actual = solution.deleteDuplicates(head.get());
  check_equal(actual, expected.get());
}

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesMultipleDuplicates) {
  auto head = ListNode::from({1, 1, 2, 3, 3, 3, 3, 3, 3});
  auto expected = ListNode::from({1, 2, 3});
  auto actual = solution.deleteDuplicates(head.get());
  check_equal(actual, expected.get());
}

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesNegativeAndPositive) {
  auto head = ListNode::from({-2, -1, -1, 0, 0, 1, 2});
  auto expected = ListNode::from({-2, -1, 0, 1, 2});
  auto actual = solution.deleteDuplicates(head.get());
  check_equal(actual, expected.get());
}

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesEmptyList) {
  auto actual = solution.deleteDuplicates(nullptr);
  check_equal(actual, nullptr);
}

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesSingleElementList) {
  auto head = ListNode::from({42});
  auto expected = ListNode::from({42});
  auto actual = solution.deleteDuplicates(head.get());
  check_equal(actual, expected.get());
}

TEST_F(RemoveDuplicatesFromSortedListTest, HandlesAllDuplicates) {
  auto head = ListNode::from({5, 5, 5, 5, 5});
  auto expected = ListNode::from({5});
  auto actual = solution.deleteDuplicates(head.get());
  check_equal(actual, expected.get());
}
