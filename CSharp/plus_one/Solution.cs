namespace plus_one {
    public class Solution {
        public static int[] PlusOne(int[] digits) {
            for (int i = digits.Length - 1; i >= 0; --i) {
                if (digits[i] == 9) {
                    digits[i] = 0;
                } else {
                    digits[i]++;
                    return digits;
                }
            }
            return digits.Prepend(1).ToArray();
        }
    }
}
