package leetcode.RemoveDuplicates2;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

class RemoveDuplicates2Test {
  protected Solution solution = new Solution();

  @Test
  @DisplayName("positive numbers only")
  void test1() {
    int[] nums = {1, 1, 1, 2, 2, 3};
    int expected = 5;
    int[] expectedNums = {1, 1, 2, 2, 3};
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
    assertArrayEquals(expectedNums, Arrays.copyOfRange(nums, 0, expected));
  }

  @Test
  @DisplayName("apear randomly")
  void test2() {
    int[] nums = {0, 0, 1, 1, 1, 1, 2, 3, 3};
    int expected = 7;
    int[] expectedNums = {0, 0, 1, 1, 2, 3, 3};
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
    assertArrayEquals(expectedNums, Arrays.copyOfRange(nums, 0, expected));
  }

  @Test
  @DisplayName("has negative numbers")
  void test3() {
    int[] nums = {-1, -1, 0, 0, 1, 1, 1, 2, 3, 3};
    int expected = 9;
    int[] expectedNums = {-1, -1, 0, 0, 1, 1, 2, 3, 3};
    int actual = solution.removeDuplicates(nums);
    assertEquals(expected, actual);
    assertArrayEquals(expectedNums, Arrays.copyOfRange(nums, 0, expected));
  }
}
