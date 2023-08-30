namespace missing_number {
    public class Solution {
        public static int MissingNumber(int[] nums) {
            int sum = nums.Sum();
            int total = nums.Length * (nums.Length + 1) / 2;
            return total - sum;
        }
    }
}