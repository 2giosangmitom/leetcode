namespace num_of_good_pairs;

public class Solution {
    public static int NumIdenticalPairs(int[] nums) {
        int result = 0;
        var hashMap = new Dictionary<int, int>();
        foreach (var num in nums) {
            if (!hashMap.ContainsKey(num)) {
                hashMap[num] = 1;
            } else {
                result += hashMap[num];
                hashMap[num] += 1;
            }
        }
        return result;
    }
}
