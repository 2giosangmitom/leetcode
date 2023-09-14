function countOdds(low: number, high: number): number {
  const length = high - low + 1;
  let result = Math.floor(length / 2);
  if (low % 2 == 1 && high % 2 == 1) {
    return ++result;
  }
  return result;
}

export default countOdds;
