package leetcode.PalindromeNumber;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

class PalindromeNumberTest {
  Solution solution = new Solution();

  @ParameterizedTest(name = "Case {index}: input = {0}, expected = true")
  @ValueSource(ints = {121, 212})
  void testPalindrome(int number) {
    assertTrue(solution.isPalindrome(number));
  }

  @ParameterizedTest(name = "Case {index}: input = {0}, expected = false")
  @ValueSource(ints = {123, 10, -121})
  void testNonPalindrome(int number) {
    assertFalse(solution.isPalindrome(number));
  }
}
