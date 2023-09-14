namespace palindrome_number;

public class Solution {
  private static int Reverse(int num) {
    int result = 0;
    while (num != 0) {
      int lastNumber = num % 10;
      result = result * 10 + lastNumber;
      num /= 10;
    }
    return result;
  }

  public static bool IsPalindrome(int x) {
    if (x < 0) {
      return false;
    }
    if (x >= 0 && x < 10) {
      return true;
    }
    return Reverse(x) == x;
  }
}
