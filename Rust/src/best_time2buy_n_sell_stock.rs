pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut buy_day = 0;
    for (sell_day, price_of_sell_day) in prices.iter().enumerate().skip(1) {
        if prices[buy_day] < *price_of_sell_day {
            let profit = price_of_sell_day - prices[buy_day];
            max_profit = max_profit.max(profit);
        } else {
            buy_day = sell_day;
        }
    }
    max_profit
}
