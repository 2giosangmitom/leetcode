function maxProfit(prices: number[]): number {
  let maxProfit = 0;
  let buyDay = 0;
  for (let sellDay = 1; sellDay < prices.length; ++sellDay) {
    if (prices[buyDay] < prices[sellDay]) {
      const profit = prices[sellDay] - prices[buyDay];
      maxProfit = Math.max(maxProfit, profit);
    } else {
      buyDay = sellDay;
    }
  }
  return maxProfit;
}

export default maxProfit;
