#include <algorithm>
#include <gtest/gtest.h>
#include <map>
#include <solution_tmpl.h>
#include <string>
#include <unordered_map>
#include <vector>
using namespace std;

class GroupAnagrams : public ::testing::Test {
protected:
  Solution solution;
};

void check_equal(vector<vector<string>> result, vector<vector<string>> want) {
  unordered_map<string, int> hash_map = {};
  for (auto pair : result) {
    bool found = false;
    for (auto pair2 : want) {
      if (pair.size() != pair2.size()) {
        continue;
      }

      map<string, int> count_left = {};
      for (string s : pair) {
        count_left[s]++;
      }

      map<string, int> count_right = {};
      for (string s : pair2) {
        count_right[s]++;
      }

      if (equal(count_left.begin(), count_left.end(), count_right.begin(),
                count_right.end())) {
        found = true;
      }
    }
    if (!found) {
      ADD_FAILURE_AT(__FILE__, __LINE__);
    }
  }
}

TEST_F(GroupAnagrams, SameLength) {
  vector<string> strs = {"eat", "tea", "tan", "ate", "nat", "bat"};
  vector<vector<string>> want = {
      {"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}};
  vector<vector<string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}

TEST_F(GroupAnagrams, EmptyString) {
  vector<string> strs = {""};
  vector<vector<string>> want = {{""}};
  vector<vector<string>> result = solution.groupAnagrams(strs);

  check_equal(result, want);
}