#include <gtest/gtest.h>
#include <solution_tmpl.h>

Solution solution;

TEST(PalindromeTest, case1) {
  int x = 121;

  bool actual = solution.isPalindrome(x);
  EXPECT_TRUE(actual);
}

TEST(PalindromeTest, case2) {
  int x = -121;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}

TEST(PalindromeTest, case3) {
  int x = 10;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}

TEST(PalindromeTest, case4) {
  int x = 1234567899;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}