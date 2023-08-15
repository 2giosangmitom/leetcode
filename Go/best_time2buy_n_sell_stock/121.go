/**
 * Runtime: 112ms (Beats 94.92%)
 * Memory: 8.3MB (Beats 55.96%)
 */

package besttime2buynsellstock

func maxProfit(prices []int) int {
	max_profit := 0
	buy_day := 0
	for sell_day := 1; sell_day < len(prices); sell_day++ {
		if prices[buy_day] < prices[sell_day] {
			profit := prices[sell_day] - prices[buy_day]
			max_profit = func(x, y int) int {
				if x < y {
					return y
				}
				return x
			}(max_profit, profit)
		} else {
			buy_day = sell_day
		}
	}
	return max_profit
}

// NOTE: this is a cleaner way to solve but LeetCode doesn't support Go 1.21 right now
func maxProfit2(prices []int) int {
	max_profit := 0
	buy_day := 0
	for sell_day := 1; sell_day < len(prices); sell_day++ {
		if prices[buy_day] < prices[sell_day] {
			profit := prices[sell_day] - prices[buy_day]
			max_profit = max(profit, max_profit)
		} else {
			buy_day = sell_day
		}
	}
	return max_profit
}
