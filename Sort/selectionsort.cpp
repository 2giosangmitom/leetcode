#include <iostream>
using namespace std;

void selectionsort(int arr[], int n) {
  for (int i = 0; i < n - 1; i++) {
    int min = i;
    for (int j = i + 1; j < n; j++) {
      if (arr[j] < arr[min]) {
        min = j;
      }
    }
    swap(arr[min], arr[i]);
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
  selectionsort(arr, n);
  for (int i = 0; i < n; i++) {
    cout << arr[i] << " ";
  }
}
