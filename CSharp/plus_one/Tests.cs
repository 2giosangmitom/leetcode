namespace plus_one;

public class Test_PlusOne {
  [Fact]
  public void Test1() {
    int[] got = Solution.PlusOne(new int[] { 1, 2, 3 });
    Assert.Equal(new int[] { 1, 2, 4 }, got);
  }

  [Fact]
  public void Test2() {
    int[] got = Solution.PlusOne(new int[] { 4, 3, 2, 1 });
    Assert.Equal(new int[] { 4, 3, 2, 2 }, got);
  }

  [Fact]
  public void Test3() {
    int[] got = Solution.PlusOne(new int[] { 9 });
    Assert.Equal(new int[] { 1, 0 }, got);
  }
}
