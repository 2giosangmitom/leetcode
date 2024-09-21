#include <gtest/gtest.h>
#include <solution_tmpl.h>

Solution solution;

TEST(ReverseIntegerTest, case1) {
  int x = 123;
  int want = 321;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST(ReverseIntegerTest, case2) {
  int x = -123;
  int want = -321;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST(ReverseIntegerTest, case3) {
  int x = 120;
  int want = 21;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

TEST(ReverseIntegerTest, case4) {
  long x = 1463847412;
  int want = 2147483641;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}

// Overflow occur
TEST(ReverseIntegerTest, case5) {
  int x = -2147483648;
  int want = 0;

  int actual = solution.reverse(x);
  EXPECT_EQ(actual, want);
}