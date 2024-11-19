package leetcode.SummaryRanges;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

class Solution {
  public List<String> summaryRanges(int[] nums) {
    if (nums.length == 0) return Arrays.asList(new String[] {});
    ArrayList<String> result = new ArrayList<>();

    int count = 1;
    for (int i = 1; i < nums.length; i++) {
      if (nums[i] != nums[i - 1] + 1) {
        if (count > 1) {
          result.add(String.format("%d->%d", nums[i - count], nums[i - 1]));
        } else {
          result.add(Integer.toString(nums[i - 1]));
        }
        count = 1;
      } else {
        count++;
      }
    }

    if (count > 1) {
      result.add(String.format("%d->%d", nums[nums.length - count], nums[nums.length - 1]));
    } else {
      result.add(Integer.toString(nums[nums.length - 1]));
    }

    return result;
  }
}
