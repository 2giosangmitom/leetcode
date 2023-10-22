function reverse(x: number): number {
  const MAX = Math.pow(2, 31) - 1;
  let result = 0;
  let isNegative = x < 0;
  if (isNegative) {
    x = Math.abs(x);
  }

  while (x != 0) {
    const lastDigit = x % 10;
    result = result * 10 + lastDigit;

    if (result > MAX || result < -MAX + 1) {
      return 0;
    }

    x = Math.floor(x / 10);
  }

  return isNegative ? -result : result;
}

export { reverse };
