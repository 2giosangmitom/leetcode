package leetcode.ValidParentheses;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ValidParenthesesTest {
  Solution solution = new Solution();

  @ParameterizedTest(name = "Case {index}: isValid({0}) = {1}")
  @CsvSource({"'()', true", "'()[]{}', true", "'(]', false", "'([])', true", "'([}}])', false"})
  void testIsValid(String s, boolean expected) {
    boolean result = solution.isValid(s);
    assertEquals(expected, result);
  }
}
