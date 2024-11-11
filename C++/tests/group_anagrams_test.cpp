#include <algorithm>
#include <group_anagrams.hpp>
#include <gtest/gtest.h>
#include <string>
#include <vector>

class GroupAnagramsTest : public ::testing::Test {
protected:
  Solution solution;
};

void check_equal(std::vector<std::vector<std::string>> result,
                 std::vector<std::vector<std::string>> want) {
  for (auto &vec : result) {
    std::sort(vec.begin(), vec.end());
  }
  for (auto &vec : want) {
    std::sort(vec.begin(), vec.end());
  }
  std::sort(result.begin(), result.end());
  std::sort(want.begin(), want.end());
  ASSERT_EQ(result, want);
}

TEST_F(GroupAnagramsTest, same_length) {
  std::vector<std::string> strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
  std::vector<std::vector<std::string>> want = {
      {"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagramsTest, empty_string) {
  std::vector<std::string> strs = {""};
  std::vector<std::vector<std::string>> want = {{""}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagramsTest, single_character_strings) {
  std::vector<std::string> strs = {"a", "b", "a"};
  std::vector<std::vector<std::string>> want = {{"a", "a"}, {"b"}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagramsTest, mixed_case_strings) {
  std::vector<std::string> strs = {"a", "A"};
  std::vector<std::vector<std::string>> want = {{"a"}, {"A"}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagramsTest, no_anagrams) {
  std::vector<std::string> strs = {"abc", "def", "ghi"};
  std::vector<std::vector<std::string>> want = {{"abc"}, {"def"}, {"ghi"}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagramsTest, all_anagrams) {
  std::vector<std::string> strs = {"abc", "bca", "cab"};
  std::vector<std::vector<std::string>> want = {{"abc", "bca", "cab"}};
  std::vector<std::vector<std::string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}
