package leetcode.BestTimeToBuyAndSellStock;

public class Solution {
  public int maxProfit(int[] prices) {
    int bestProfit = 0;
    int sellDay = 0;

    for (int i = 1; i < prices.length; i++) {
      if (prices[i] > prices[sellDay]) {
        bestProfit = Math.max(bestProfit, prices[i] - prices[sellDay]);
      } else {
        sellDay = i;
      }
    }

    return bestProfit;
  }
}
