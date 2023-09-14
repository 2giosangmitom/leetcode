namespace count_odd_nums;

public class CountOddNums {
  [Fact]
  public void Test1() {
    int got = Solution.CountOdds(3, 7);
    Assert.Equal(3, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.CountOdds(8, 10);
    Assert.Equal(1, got);
  }

  [Fact]
  public void Test3() {
    int got = Solution.CountOdds(21, 22);
    Assert.Equal(1, got);
  }

  [Fact]
  public void Test4() {
    int got = Solution.CountOdds(8, 13);
    Assert.Equal(3, got);
  }
}
