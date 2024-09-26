package leetcode.TwoSum;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;

import org.junit.jupiter.api.Test;

class TwoSumTest {
  Solution solution = new Solution();

  @Test
  void case_1() {
    int[] nums = new int[] {2, 7, 11, 15};
    int target = 9;
    int[] want = new int[] {0, 1};
    int[] result = solution.twoSum(nums, target);
    assertArrayEquals(want, result);
  }

  @Test
  void case_2() {
    int[] nums = new int[] {3, 2, 4};
    int target = 6;
    int[] want = new int[] {1, 2};
    int[] result = solution.twoSum(nums, target);
    assertArrayEquals(want, result);
  }

  @Test
  void case_3() {
    int[] nums = new int[] {3, 3};
    int target = 6;
    int[] want = new int[] {0, 1};
    int[] result = solution.twoSum(nums, target);
    assertArrayEquals(want, result);
  }
}
