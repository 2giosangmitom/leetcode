namespace best_time_to_buy_and_sell_stock;

public class Test_BestTimeToBuyAndSellStock {
  [Fact]
  public void Test1() {
    int got = Solution.MaxProfit(new int[] { 7, 1, 5, 3, 6, 4 });
    Assert.Equal(5, got);
  }

  [Fact]
  public void Test2() {
    int got = Solution.MaxProfit(new int[] { 7, 6, 4, 3, 1 });
    Assert.Equal(0, got);
  }
}
