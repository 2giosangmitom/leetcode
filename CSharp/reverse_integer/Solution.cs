namespace reverse_integer {
    public class Solution {
        public static int Reverse(int x) {
            int result = 0;
            while (x != 0) {
                int lastNumber = x % 10;
                try {
                    result = checked(result * 10 + lastNumber);
                } catch (OverflowException) {
                    return 0;
                }
                x /= 10;
            }
            return result;
        }
    }
}