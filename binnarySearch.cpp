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
int main() {}
