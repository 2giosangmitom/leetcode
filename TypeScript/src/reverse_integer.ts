export function reverse(x: number): number {
  const MAX = 2 ** 31 - 1;
  let result = 0;
  const isNegative = x < 0;

  if (isNegative) {
    x = Math.abs(x);
  }

  while (x !== 0) {
    const lastDigit = x % 10;
    result = result * 10 + lastDigit;
    x = Math.floor(x / 10);
  }

  if (result > MAX) {
    return 0;
  }

  return isNegative ? -result : result;
}
