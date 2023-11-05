namespace CSharp.TwoSum;

public class Solution {
    public static int[] TwoSum(int[] nums, int target) {
        Dictionary<int, int> map = new();
        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];
            if (map.ContainsKey(complement)) {
                return new int[] { map[complement], i };
            } else {
                map.Add(nums[i], i);
            }
        }
        return new int[] { -1 };
    }

    public static int[] TwoSum2(int[] nums, int target) {
        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];
            for (int j = i + 1; j < nums.Length; j++) {
                if (complement == nums[j]) {
                    return new int[] { i, j };
                }
            }
        }
        return new int[] { -1 };
    }
}