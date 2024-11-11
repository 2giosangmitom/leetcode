/**
 * @param {number} numBottles
 * @param {number} numExchange
 * @return {number}
 */
function numWaterBottles(numBottles, numExchange) {
  let result = 0;

  while (numBottles >= numExchange) {
    const rest = numBottles % numExchange;
    result += numBottles - rest;
    numBottles = Math.floor(numBottles / numExchange) + rest;
  }

  result += numBottles;

  return result;
}

export { numWaterBottles };
