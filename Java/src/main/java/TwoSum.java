import java.util.HashMap;

class TwoSum {
    public static int[] twoSum(int[] nums, int target) {
        HashMap<Integer, Integer> hashMap = new HashMap<>();
        for (int i = 0; i < nums.length; ++i) {
            int complement = target - nums[i];
            if (hashMap.containsKey(complement)) {
                return new int[]{hashMap.get(complement), i};
            } else {
                hashMap.put(nums[i], i);
            }
        }
        return new int[]{-1};
    }

    public static int[] twoSum2(int[] nums, int target) {
        for (int i = 0; i < nums.length; ++i) {
            int complement = target - nums[i];
            for (int j = i + 1; j < nums.length; ++j) {
                if (complement == nums[j]) {
                    return new int[]{i, j};
                }
            }
        }
        return new int[]{-1};
    }
}