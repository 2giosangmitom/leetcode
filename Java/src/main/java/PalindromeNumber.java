import java.util.function.Consumer;
import java.util.function.Function;

public class PalindromeNumber {
  public static boolean isPalindrome(int x) {
    if (x < 0) {
      return false;
    }

    Function<Integer, Integer> reverse =
        (num) -> {
          int result = 0;

          while (num != 0) {
            int lastDigit = num % 10;
            result = result * 10 + lastDigit;
            num /= 10;
          }

          return result;
        };

    return x == reverse.apply(x);
  }
}
