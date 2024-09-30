#include <gtest/gtest.h>
#include <solution_tmpl.h>

class ReverseIntegerTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(ReverseIntegerTest, ReversePositiveInteger) {
  int x = 123;
  int want = 321;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST_F(ReverseIntegerTest, ReverseNegativeInteger) {
  int x = -123;
  int want = -321;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST_F(ReverseIntegerTest, ReverseIntegerWithTrailingZeros) {
  int x = 120;
  int want = 21;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST_F(ReverseIntegerTest, ReverseIntegerWithPotentialOverflow) {
  int x = 1463847412;
  int want = 2147483641;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST_F(ReverseIntegerTest, ReverseMinimumNegativeInteger) {
  int x = -2147483648;
  int want = 0;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}
