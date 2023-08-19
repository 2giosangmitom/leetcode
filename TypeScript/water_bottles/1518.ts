/**
 * Runtime: 64ms (Beats 26.9%)
 * Memory: 42.9MB (Beats 47.83%)
 */

function numWaterBottles(numBottles: number, numExchange: number): number {
  let result = 0;
  while (numBottles >= numExchange) {
    const rest = numBottles % numExchange;
    result = result + numBottles - rest;
    numBottles = Math.floor(numBottles / numExchange) + rest;
  }
  return result + numBottles;
}

export default numWaterBottles;
