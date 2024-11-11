package leetcode.ValidParentheses;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ValidParenthesesTest {
  protected Solution solution = new Solution();

  @ParameterizedTest(name = "{2}")
  @CsvSource({
    "'()', true, valid simple parentheses",
    "'()[]{}', true, valid multiple types",
    "'(]', false, invalid mixed parentheses",
    "'([])', true, valid nested parentheses",
    "'([}}])', false, invalid nested parentheses",
    "'(', false, invalid single opening bracket",
    "')', false, invalid single closing bracket"
  })
  void testIsValid(String s, boolean want, String name) {
    boolean actual = solution.isValid(s);
    assertEquals(want, actual);
  }
}
