#include <gtest/gtest.h>
#include <solution_tmpl.h>
#include <string>
#include <vector>
using namespace std;

class GroupAnagrams : public ::testing::Test {
protected:
  Solution solution;
};

void check_equal(vector<vector<string>> result, vector<vector<string>> want) {
  for (size_t i = 0; i < result.size(); i++) {
    for (size_t j = 0; j < result[i].size(); j++) {
      string target = result[i][j];
      bool found = false;
      for (size_t r = 0; r < want[i].size(); r++) {
        if (want[i][r] == target) {
          found = true;
          break;
        }
      }
      if (!found) {
        ADD_FAILURE_AT(__FILE__, __LINE__)
            << "Mismatch at result[" << i << "][" << j << "]: " << target
            << " not found in want[" << i << "]";
      }
    }
  }
}

TEST_F(GroupAnagrams, SameLengths) {
  vector<string> strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
  vector<vector<string>> want = {
      {"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}};
  vector<vector<string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagrams, EmptyStrings) {
  vector<string> strs = {""};
  vector<vector<string>> want = {{""}};
  vector<vector<string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}