function reverse(x: number): number {
  const MAX = 2147483647;
  let result = 0;
  const isNegative = x < 0;

  if (isNegative) {
    x = x * -1;
  }

  while (x !== 0) {
    const lastNumber = x % 10;
    result = result * 10 + lastNumber;
    x = Math.floor(x / 10);
  }

  if (result > MAX) {
    return 0;
  }

  return isNegative ? -result : result;
}

export default reverse;
