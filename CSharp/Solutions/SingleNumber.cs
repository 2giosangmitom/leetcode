namespace Solutions;

public class SingleNumber {
    public static int SingleNumber1(int[] nums) {
        Dictionary<int, int> map = [];
        for (int i = 0; i < nums.Length; i++) {
            if (map.TryGetValue(nums[i], out int value)) {
                map[nums[i]] = value + 1;
            } else {
                map.Add(nums[i], 1);
            }
        }

        for (int i = 0; i < nums.Length; i++) {
            if (map[nums[i]] == 1) {
                return nums[i];
            }
        }

        return -1;
    }
}