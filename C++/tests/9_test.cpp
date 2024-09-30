#include <gtest/gtest.h>
#include <solution_tmpl.h>

class PalindromeTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(PalindromeTest, PositivePalindrome) {
  int x = 121;

  bool actual = solution.isPalindrome(x);
  EXPECT_TRUE(actual);
}

TEST_F(PalindromeTest, NegativePalindrome) {
  int x = -121;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}

TEST_F(PalindromeTest, NotPalindromeEndingInZero) {
  int x = 10;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}

TEST_F(PalindromeTest, NotPalindromeLargeNumber) {
  int x = 1234567899;

  bool actual = solution.isPalindrome(x);
  EXPECT_FALSE(actual);
}
