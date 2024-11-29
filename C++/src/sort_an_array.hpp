#ifndef SORT_AN_ARRAY_HPP
#define SORT_AN_ARRAY_HPP

#include <vector>
using namespace std;

class Solution {
public:
  void merge(vector<int> &nums, int p, int q, int m);
  void merge_sort(vector<int> &nums, int p, int q);
  vector<int> sortArray(vector<int> &nums);
};

#endif // SORT_AN_ARRAY_HPP
