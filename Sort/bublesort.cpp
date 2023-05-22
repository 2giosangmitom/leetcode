#include <iostream>
using namespace std;

void bubbleSort(int arr[], int n) {
  bool haveSwap = false;
  for (int i = 0; i < n - 1; ++i) {
    for (int j = 0; j < n - i - 1; ++j) {
      if (arr[j] > arr[j + 1]) {
        swap(arr[j], arr[j + 1]);
        haveSwap = true;
      }
    }
    if (haveSwap == false) {
      break;
    }
  }
}

int main() {
  int n;
  cout << "Length of arr = ";
  cin >> n;
  int arr[n];
  for (int i = 0; i < n; i++) {
    cout << "arr[" << i << "] = ";
    cin >> arr[i];
  }
  bubbleSort(arr, n);
  for (int i = 0; i < n; i++) {
    cout << arr[i] << " ";
  }
  return 0;
}
