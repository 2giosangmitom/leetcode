import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

public class PalindromeNumberTests {
  @ParameterizedTest(name = "isPalindrome({0})")
  @CsvSource({"-10,false", "5,true", "121,true", "321,false", "111,true"})
  void test(int x, boolean want) {
    boolean result = PalindromeNumber.isPalindrome(x);
    assertEquals(want, result);
  }
}
