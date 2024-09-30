package leetcode.ValidParentheses;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ValidParenthesesTest {
  Solution solution = new Solution();

  @ParameterizedTest
  @CsvSource({"'()', true", "'()[]{}', true", "'(]', false", "'([])', true", "'([}}])', false"})
  void manyTests(String s, boolean want) {
    boolean result = solution.isValid(s);
    assertEquals(want, result);
  }
}
