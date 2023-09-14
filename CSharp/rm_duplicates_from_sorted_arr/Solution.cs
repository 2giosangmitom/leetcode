namespace remove_duplicates_from_sorted_array;

public class Solution {
    public static int RemoveDuplicates(int[] nums) {
        int k = 0;
        for (int i = 1; i < nums.Length; ++i) {
            if (nums[k] != nums[i]) {
                nums[++k] = nums[i];
            }
        }
        return k + 1;
    }
}
