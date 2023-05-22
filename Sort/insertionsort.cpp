#include <iostream>
using namespace std;

void sort(int arr[], int n) {
  int temp, j;
  for (int i = 1; i < n; i++) {
    temp = arr[i];
    j = i - 1;
    while (j >= 0 && arr[j] > temp) {
      arr[j + 1] = arr[j];
      j--;
    }
    arr[j + 1] = temp;
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
  sort(arr, n);
  for (int i = 0; i < n; i++) {
    cout << arr[i] << " ";
  }
}
