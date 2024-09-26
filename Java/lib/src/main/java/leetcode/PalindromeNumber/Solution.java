package leetcode.PalindromeNumber;

class Solution {
  public boolean isPalindrome(int x) {
    if (x < 0) {
      return false;
    }

    int temp = x;
    long reversed = 0;
    while (temp != 0) {
      int lastDigit = temp % 10;
      reversed = reversed * 10 + lastDigit;
      temp /= 10;
    }

    return reversed == x;
  }
}
