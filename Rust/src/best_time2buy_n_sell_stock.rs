/**
 * Runtime: 10ms (Beats 86.19%)
 * Memory: 2.9MB (Beats 60.61%)
 */

pub struct Solution;

pub trait BestTimeToBuyAndSellStock {
    fn max_profit(prices: Vec<i32>) -> i32;
}

impl BestTimeToBuyAndSellStock for Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut buy_day = 0;
        for (sell_day, price_of_sell_day) in prices.iter().enumerate().skip(1) {
            if prices[buy_day] < *price_of_sell_day {
                let profit = price_of_sell_day - prices[buy_day];
                max_profit = profit.max(max_profit);
            } else {
                buy_day = sell_day;
            }
        }
        max_profit
    }
}
