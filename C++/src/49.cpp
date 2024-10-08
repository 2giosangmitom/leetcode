#include <algorithm>
#include <solution_tmpl.h>
#include <unordered_map>

vector<vector<string>> Solution::groupAnagrams(vector<string> &strs) {
  unordered_map<string, vector<string>> hash_map;

  for (string s : strs) {
    string word = s;
    sort(word.begin(), word.end());
    hash_map[word].push_back(s);
  }

  vector<vector<string>> result;
  for (auto pair : hash_map) {
    result.push_back(pair.second);
  }

  return result;
}