#include <gtest/gtest.h>
#include <solution_tmpl.h>

Solution solution;

TEST(LongestCommonPrefixTest, case1) {
  vector<string> strs = {"flower", "flow", "flight"};
  string want = "fl";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST(LongestCommonPrefixTest, case2) {
  vector<string> strs = {"dog", "racecar", "car"};
  string want = "";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}
