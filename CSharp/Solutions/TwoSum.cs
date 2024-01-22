namespace Solutions;

public class TwoSum {
    public static int[] TwoSum1(int[] nums, int target) {
        Dictionary<int, int> map = [];
        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];
            if (map.TryGetValue(complement, out int value)) {
                return [value, i];
            } else {
                map.Add(nums[i], i);
            }
        }
        return [-1];
    }

    public static int[] TwoSum2(int[] nums, int target) {
        for (int i = 0; i < nums.Length; i++) {
            int complement = target - nums[i];
            for (int j = i + 1; j < nums.Length; j++) {
                if (complement == nums[j]) {
                    return [i, j];
                }
            }
        }
        return [-1];
    }
}