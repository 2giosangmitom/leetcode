#include <iostream>
using namespace std;

int linearSearch(int arr[], int n, int target) {
  for (int i = 0; i < n; i++) {
    if (arr[i] == target) {
      return i;
    }
  }
  return -1;
}

int main() {
  int n, target;
  cout << "Length of arr = ";
  cin >> n;
  int arr[n];
  for (int i = 0; i <= n - 1; i++) {
    cout << "arr[" << i << "] = ";
    cin >> arr[i];
  }
  cout << "Search: ";
  cin >> target;
  int result = linearSearch(arr, n, target);
  if (result == -1) {
    cout << "Can't find " << target;
  } else {
    cout << "Position is " << result;
  }
}
