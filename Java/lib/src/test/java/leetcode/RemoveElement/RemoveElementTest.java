package leetcode.RemoveElement;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Arrays;

import org.junit.jupiter.api.Test;

class RemoveElementTest {
  private Solution solution = new Solution();

  @Test
  void case1() {
    int[] nums = { 3, 2, 2, 3 };
    int val = 3;
    int result = solution.removeElement(nums, val);

    assertEquals(2, result);
    int[] postArr = Arrays.copyOfRange(nums, 0, result);
    Arrays.sort(postArr);
    assertArrayEquals(new int[] { 2, 2 }, postArr);
  }

  @Test
  void case2() {
    int[] nums = { 0, 1, 2, 2, 3, 0, 4, 2 };
    int val = 2;
    int result = solution.removeElement(nums, val);

    assertEquals(5, result);
    int[] postArr = Arrays.copyOfRange(nums, 0, result);
    Arrays.sort(postArr);
    assertArrayEquals(new int[] { 0, 0, 1, 3, 4 }, postArr);
  }
}
