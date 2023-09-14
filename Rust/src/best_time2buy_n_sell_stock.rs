struct Solution;

trait BestTimeToBuyAndSellStock {
    fn max_profit(prices: Vec<i32>) -> i32;
}

impl BestTimeToBuyAndSellStock for Solution {
    fn max_profit(prices: Vec<i32>) -> i32 {
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
}

#[test]
fn test_best_time_to_buy_and_sell_stock() {
    struct Tt {
        prices: Vec<i32>,
        want: i32,
    }

    let cases: Vec<Tt> = vec![
        Tt {
            prices: vec![7, 1, 5, 3, 6, 4],
            want: 5,
        },
        Tt { prices: vec![7, 6, 4, 3, 1], want: 0 },
        Tt { prices: vec![2, 1, 4], want: 3 },
    ];

    for t in cases.into_iter() {
        let result = Solution::max_profit(t.prices);
        assert_eq!(result, t.want);
    }
}
