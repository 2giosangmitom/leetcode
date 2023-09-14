namespace best_time_to_buy_and_sell_stock;

public class Solution {
    public static int MaxProfit(int[] prices) {
        int maxProfit = 0;
        int buyDay = 0;
        for (int sellDay = 1; sellDay < prices.Length; ++sellDay) {
            if (prices[buyDay] < prices[sellDay]) {
                int profit = prices[sellDay] - prices[buyDay];
                maxProfit = Math.Max(profit, maxProfit);
            } else {
                buyDay = sellDay;
            }
        }
        return maxProfit;
    }
}
