package leetcode.MissingNumber;

import java.util.Arrays;

class Solution {
  public int missingNumber(int[] nums) {
    int total = nums.length * (nums.length + 1) / 2;
    return total - Arrays.stream(nums).sum();
  }
}
