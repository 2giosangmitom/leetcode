namespace Solutions;

public class PalindromeNumber {
    public static bool IsPalindrome(int x) {
        if (x < 0) {
            return false;
        }

        static int reverse(int n) {
            int result = 0;
            while (n != 0) {
                int lastDigit = n % 10;
                result = (result * 10) + lastDigit;
                n /= 10;
            }
            return result;
        }

        return reverse(x) == x;
    }
}