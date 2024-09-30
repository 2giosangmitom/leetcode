package leetcode.ReverseInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ReverseTest {
  Solution solution = new Solution();

  @ParameterizedTest(name = "Case {index}: reverse({0}) = {1}")
  @CsvSource({
    "123, 321",
    "-123, -321",
    "120, 21",
    "1463847412, 2147483641",
    "-2147483648, 0",
  })
  void testReverse(int x, int expected) {
    int result = solution.reverse(x);
    assertEquals(expected, result);
  }
}
