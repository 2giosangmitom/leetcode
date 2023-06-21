function myPow(x: number, n: number): number {
  if (n === 0) return 1;
  const pow = Math.abs(n);
  let result: number;
  if (pow % 2 === 0) {
    result = myPow(x * x, pow / 2);
  } else {
    result = myPow(x * x, (pow - 1) / 2) * x;
  }
  return n < 0 ? 1 / result : result;
}
