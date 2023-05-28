// Best Time to Buy and Sell Stock
const maxProfit = function (prices) {
	let buyDay = 0;
	let maxProfit = 0;
	for (let i = 1; i < prices.length; ++i) {
		if (prices[i] > prices[buyDay]) {
			const profit = prices[i] - prices[buyDay];
			maxProfit = Math.max(maxProfit, profit);
		} else {
			buyDay = i;
		}
	}
	return maxProfit;
};
