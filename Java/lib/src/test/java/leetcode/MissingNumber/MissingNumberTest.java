package leetcode.MissingNumber;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class MissingNumberTest {
  protected Solution solution = new Solution();

  @Test
  @DisplayName("sample test 1")
  void test1() {
    int[] nums = {3, 0, 1};
    int expected = 2;
    int actual = solution.missingNumber(nums);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("sample test 2")
  void test2() {
    int[] nums = {0, 1};
    int expected = 2;
    int actual = solution.missingNumber(nums);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("sample test 3")
  void test3() {
    int[] nums = {9, 6, 4, 2, 3, 5, 7, 0, 1};
    int expected = 8;
    int actual = solution.missingNumber(nums);
    assertEquals(expected, actual);
  }
}
