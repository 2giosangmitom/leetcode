// Best Time to Buy and Sell Stock
use std::cmp;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut buy_day = 0;
        for sell_day in 1..prices.len() {
            if prices[buy_day] < prices[sell_day] {
                let profit = prices[sell_day] - prices[buy_day];
                max_profit = cmp::max(max_profit, profit);
            } else {
                buy_day = sell_day;
            }
        }
        max_profit
    }
}
