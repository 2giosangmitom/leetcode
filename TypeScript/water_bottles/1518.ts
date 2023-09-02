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