package leetcode.ReverseInteger;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

class ReverseIntegerTest {
  protected Solution solution = new Solution();

  @ParameterizedTest(name = "{2}")
  @CsvSource({
    "123,321,positive integer",
    "-123,-321,negative integer",
    "120,21,integer with trailing zeros",
    "-2147483648,0,encounter overflow",
    "2147483647,0,encounter overflow",
  })
  void testReverseInteger(int num, int want, String displayName) {
    int actual = solution.reverse(num);
    assertEquals(want, actual);
  }
}
