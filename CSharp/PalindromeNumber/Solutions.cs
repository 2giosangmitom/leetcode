namespace CSharp.PalindromeNumber;

public class Solution {
    public static bool IsPalindrome(int x) {
        if (x < 0) {
            return false;
        }

        static int reverse(int val) {
            int result = 0;
            while (val != 0) {
                int lastDigit = val % 10;
                result = (result * 10) + lastDigit;
                val /= 10;
            }
            return result;
        }

        return reverse(x) == x;
    }
}