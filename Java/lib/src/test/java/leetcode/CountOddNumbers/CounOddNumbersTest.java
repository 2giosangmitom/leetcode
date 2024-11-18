package leetcode.CountOddNumbers;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

public class CounOddNumbersTest {
  @ParameterizedTest(name = "{3}")
  @CsvSource({
    "3, 7, 3, odd low and high",
    "8, 10, 1, even low and high",
    "1, 10, 5, odd low and even high",
    "2, 11, 5, even low and odd high",
  })
  void countOdds(int low, int high, int expected, String displayName) {
    Solution solution = new Solution();
    int result = solution.countOdds(low, high);
    assertEquals(expected, result);
  }
}
