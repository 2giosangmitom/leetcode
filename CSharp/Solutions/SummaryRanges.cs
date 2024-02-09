namespace Solutions;

public class SummaryRanges {
    public static IList<string> SummaryRanges1(int[] nums) {
        List<string> result = [];

        for (int i = 0; i < nums.Length; i++) {
            int end = i;
            while (end + 1 < nums.Length && nums[end + 1] - nums[end] == 1) {
                end++;
            }

            if (end > i) {
                result.Add($"{nums[i]}->{nums[end]}");
            } else {
                result.Add($"{nums[end]}");
            }

            i = end;
        }

        return result;
    }
}