#include <gtest/gtest.h>
#include <solution_tmpl.h>
using namespace std;

Solution solution;

TEST(ValidParenthesesTest, case1) {
  string s = "()";

  EXPECT_TRUE(solution.isValid(s));
}

TEST(ValidParenthesesTest, case2) {
  string s = "()[]{}";

  EXPECT_TRUE(solution.isValid(s));
}

TEST(ValidParenthesesTest, case3) {
  string s = "(]";

  EXPECT_FALSE(solution.isValid(s));
}

TEST(ValidParenthesesTest, case4) {
  string s = "([])";

  EXPECT_TRUE(solution.isValid(s));
}

TEST(ValidParenthesesTest, case5) {
  string s = "([}}])";

  EXPECT_FALSE(solution.isValid(s));
}