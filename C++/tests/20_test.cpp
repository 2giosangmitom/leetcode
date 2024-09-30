#include <gtest/gtest.h>
#include <solution_tmpl.h>
using namespace std;

class ValidParenthesesTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(ValidParenthesesTest, ValidSimpleParentheses) {
  string s = "()";
  EXPECT_TRUE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, ValidMultipleTypes) {
  string s = "()[]{}";
  EXPECT_TRUE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, InvalidMixedParentheses) {
  string s = "(]";
  EXPECT_FALSE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, ValidNestedParentheses) {
  string s = "([])";
  EXPECT_TRUE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, InvalidNestedParentheses) {
  string s = "([}}])";
  EXPECT_FALSE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, InvalidSingleOpeningBracket) {
  string s = "(";
  EXPECT_FALSE(solution.isValid(s));
}

TEST_F(ValidParenthesesTest, InvalidSingleClosingBracket) {
  string s = ")";
  EXPECT_FALSE(solution.isValid(s));
}
