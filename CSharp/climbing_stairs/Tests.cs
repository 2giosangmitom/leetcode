namespace climbing_stairs;

public class Test_ClimbingStairs {
  [Fact]
  public void Test1() {
    int got = Solution.ClimbStairs(2);
    Assert.Equal(2, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.ClimbStairs(4);
    Assert.Equal(5, got);
  }

  [Fact]
  public void Test3() {
    int got = Solution.ClimbStairs(3);
    Assert.Equal(3, got);
  }
}
