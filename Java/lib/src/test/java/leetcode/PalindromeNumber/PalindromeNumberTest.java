package leetcode.PalindromeNumber;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class PalindromeNumberTest {
  protected Solution solution = new Solution();

  @ParameterizedTest(name = "{2}")
  @CsvSource({
    "121, true, positive palindrome",
    "-121, false, negative palindrome",
    "10, false, not palindrome ending in zero",
    "1234567899, false, not palindrome large number"
  })
  void testIsPalindrome(int x, boolean want, String name) {
    boolean actual = solution.isPalindrome(x);
    assertEquals(want, actual);
  }
}
