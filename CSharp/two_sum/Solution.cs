namespace two_sum {
    public class Solution {
        public static int[] TwoSum(int[] nums, int target) {
            var hashMap = new Dictionary<int, int>();
            for (int i = 0; i < nums.Length; i++) {
                int needNumber = target - nums[i];
                if (hashMap.ContainsKey(needNumber)) {
                    return new int[] { hashMap[needNumber], i };
                } else {
                    hashMap[nums[i]] = i;
                }
            }
            return new int[] { -1 };
        }
    }
}
