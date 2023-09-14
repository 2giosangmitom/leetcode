namespace climbing_stairs;

public class Solution {
  public static int ClimbStairs(int n) {
    if (n == 1 || n == 2) return n;
    int a = 1, b = 2;
    for (int i = 3; i < n; i++) {
      int temp = b;
      b += a;
      a = temp;
    }
    return a + b;
  }
}
