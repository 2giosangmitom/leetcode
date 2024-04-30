#[cfg(test)]
mod tests {
    use leetcode::best_time2buy_n_sell_stock::*;

    #[test]
    fn test_best_time_to_buy_and_sell_stock_case1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let want = 5;

        let result = max_profit(prices);

        assert_eq!(result, want);
    }

    #[test]
    fn test_best_time_to_buy_and_sell_stock_case2() {
        let prices = vec![7, 6, 4, 3, 1];
        let want = 0;

        let result = max_profit(prices);

        assert_eq!(result, want);
    }

    #[test]
    fn test_best_time_to_buy_and_sell_stock_case3() {
        let prices = vec![2, 1, 4];
        let want = 3;

        let result = max_profit(prices);

        assert_eq!(result, want);
    }

    #[test]
    fn test_best_time_to_buy_and_sell_stock_case4() {
        let prices = vec![1, 5, 2];
        let want = 4;

        let result = max_profit(prices);

        assert_eq!(result, want);
    }
}
