// Best Time to Buy and Sell Stock
function maxProfit(prices: number[]): number {
	let maxProfit = 0;
	let buyDay = 0;
	for (let i = 1; i < prices.length; ++i) {
		if (prices[i] > prices[buyDay]) {
			const profit = prices[i] - prices[buyDay];
			maxProfit = Math.max(profit, maxProfit);
		} else {
			buyDay = i;
		}
	}
	return maxProfit;
}
