namespace count_odd_nums;

public class Solution {
    public static int CountOdds(int low, int high) {
        int length = high - low + 1;
        int result = length / 2;
        if (low % 2 == 1 && high % 2 == 1) {
            return ++result;
        }
        return result;
    }
}
