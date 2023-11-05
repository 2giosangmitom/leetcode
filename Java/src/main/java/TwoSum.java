import java.util.HashMap;

class Solution {
  public static int[] twoSum(int[] nums, int target) {
    HashMap<Integer, Integer> hashMap = new HashMap<>();
    for (int i = 0; i < nums.length; ++i) {
      int complement = target - nums[i];
      if (hashMap.containsKey(complement)) {
        return new int[] {hashMap.get(complement), i};
      } else {
        hashMap.put(nums[i], i);
      }
    }
    return new int[] {-1};
  }
}
