package leetcode.RemoveDuplicates2;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class RemoveDuplicates2Test {
  protected Solution solution = new Solution();

  @Test
  @DisplayName("positive numbers only")
  void test1() {
    int[] nums = { 1, 1, 1, 2, 2, 3 };
    int expected = 5;
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("apear randomly")
  void test2() {
    int[] nums = { 0, 0, 1, 1, 1, 1, 2, 3, 3 };
    int expected = 7;
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
  }

  @Test
  @DisplayName("has negative numbers")
  void test3() {
    int[] nums = { -1, -1, 0, 0, 1, 1, 1, 2, 3, 3 };
    int expected = 9;
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
  }
}
