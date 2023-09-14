namespace missing_number;

public class Test_MissingNumber {
  [Fact]
  public void Test1() {
    int got = Solution.MissingNumber(new int[] { 3, 0, 1 });
    Assert.Equal(2, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.MissingNumber(new int[] { 0, 1 });
    Assert.Equal(2, got);
  }

  [Fact]
  public void Test3() {
    int got = Solution.MissingNumber(new int[] { 9, 6, 4, 2, 3, 5, 7, 0, 1 });
    Assert.Equal(8, got);
  }
}
