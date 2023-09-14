namespace sqrt;

public class Test_Sqrt {
  [Fact]
  public void Test1() {
    int got = Solution.MySqrt(4);
    Assert.Equal(2, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.MySqrt(8);
    Assert.Equal(2, got);
  }
}
