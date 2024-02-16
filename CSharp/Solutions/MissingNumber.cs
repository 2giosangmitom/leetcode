namespace Solutions;

public class MissingNumber {
    public static int MissingNumber1(int[] nums) {
        int sum = nums.Sum();
        int total = nums.Length * (nums.Length + 1) / 2;
        return total - sum;
    }
}