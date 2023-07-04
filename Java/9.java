class Solution {
  private int reverseNumber(int x) {
    int temp = x;
    int reversed = 0;
    while (temp != 0) {
      int digit = temp % 10;
      reversed = reversed * 10 + digit;
      temp /= 10;
    }
    return reversed;
  }

  public boolean isPalindrome(int x) {
    if (x < 0) return false;
    if (x <= 9 && x >= 0) return true;
    int reverseX = reverseNumber(x);
    return reverseX == x;
  }

  public static void main(String[] args) {
    Solution test = new Solution();
    System.out.println(test.isPalindrome(121)); // TEST: => true
    System.out.println(test.isPalindrome(-121)); // TEST: => false
    System.out.println(test.isPalindrome(10)); // TEST: => false
    System.out.println(test.isPalindrome(9)); // TEST: => true
  }
}
