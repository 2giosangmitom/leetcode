#include <gtest/gtest.h>
#include <solution_tmpl.h>
using namespace std;

class LongestCommonPrefixTest : public ::testing::Test {
protected:
  Solution solution;
};

TEST_F(LongestCommonPrefixTest, CommonPrefixExists) {
  vector<string> strs = {"flower", "flow", "flight"};
  string want = "fl";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, NoCommonPrefix) {
  vector<string> strs = {"dog", "racecar", "car"};
  string want = "";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, SingleString) {
  vector<string> strs = {"alone"};
  string want = "alone";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, EmptyInput) {
  vector<string> strs = {};
  string want = "";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, NoCommonPrefixAtAll) {
  vector<string> strs = {"abc", "def", "ghi"};
  string want = "";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, AllStringsIdentical) {
  vector<string> strs = {"same", "same", "same"};
  string want = "same";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}

TEST_F(LongestCommonPrefixTest, PrefixIsTheShortestString) {
  vector<string> strs = {"ab", "abc", "abcd"};
  string want = "ab";

  string actual = solution.longestCommonPrefix(strs);
  EXPECT_EQ(actual, want);
}
