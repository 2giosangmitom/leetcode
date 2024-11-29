#include <sort_an_array.hpp>
using namespace std;

void Solution::merge(vector<int> &nums, int p, int q, int m) {
  int nl = m - p + 1;
  int nr = q - m;
  vector<int> left(nl);
  vector<int> right(nr);

  for (int i = 0; i < nl; i++) {
    left[i] = nums[p + i];
  }

  for (int i = 0; i < nr; i++) {
    right[i] = nums[m + 1 + i];
  }

  int i = 0, j = 0, k = p;

  while (i < nl && j < nr) {
    if (left[i] <= right[j]) {
      nums[k] = left[i];
      i++;
    } else {
      nums[k] = right[j];
      j++;
    }
    k++;
  }

  while (i < nl) {
    nums[k] = left[i];
    i++;
    k++;
  }

  while (j < nr) {
    nums[k] = right[j];
    j++;
    k++;
  }
};

void Solution::merge_sort(vector<int> &nums, int p, int q) {
  if (p < q) {
    int m = p + (q - p) / 2;
    merge_sort(nums, p, m);
    merge_sort(nums, m + 1, q);
    merge(nums, p, q, m);
  }
};

vector<int> Solution::sortArray(vector<int> &nums) {
  merge_sort(nums, 0, (int)nums.size() - 1);
  return nums;
}
