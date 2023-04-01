#include <iostream>
using namespace std;

int binarySearch(int arr[], int left, int right, int x) {
  int middle;
  while (left <= right) {
    middle = (left + right) / 2;
    if (arr[middle] == x) {
      return middle;
    }
    if (x > arr[middle]) {
      left = middle + 1;
    } else {
      right = middle - 1;
    }
  }
  return -1;
}

int main() {
  int n;
  cout << "Length of arr = ";
  cin >> n;
  int arr[n];
  for (int i = 0; i <= n - 1; i++) {
    cout << "arr[" << i << "] = ";
    cin >> arr[i];
  }
  int target;
  cout << "Search: ";
  cin >> target;
  int result = binarySearch(arr, 0, n - 1, target);
  if (result == -1) {
    cout << "Can't find " << target;
  } else {
    cout << "Position is " << result;
  }
}
