namespace water_bottles;

public class Test_WaterBottles {
  [Fact]
  public void Test1() {
    int got = Solution.NumWaterBottles(9, 3);
    Assert.Equal(13, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.NumWaterBottles(15, 4);
    Assert.Equal(19, got);
  }
}
