import java.util.Arrays;
import java.util.HashMap;

class Solution {
  public int[] twoSum(int[] nums, int target) {
    var hashMap = new HashMap<Integer, Integer>();
    for (var i = 0; i < nums.length; ++i) {
      var needNumber = target - nums[i];
      if (hashMap.containsKey(needNumber)) {
        return new int[] {hashMap.get(needNumber), i};
      }
      hashMap.put(nums[i], i);
    }
    return new int[] {-1};
  }

  public static void main(String[] args) {
    var s = new Solution();
    int[] nums = {2, 7, 11, 15};
    System.out.println(Arrays.toString(s.twoSum(nums, 9))); // TEST: => [0, 1]
    int[] nums2 = {3, 2, 4};
    System.out.println(Arrays.toString(s.twoSum(nums2, 6))); // TEST: => [1, 2]
    int[] nums3 = {3, 2};
    System.out.println(Arrays.toString(s.twoSum(nums3, 6))); // TEST: => [0, 1]
  }
}
