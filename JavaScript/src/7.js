/**
 * @param {number} x
 * @return {number}
 */
function reverse(x) {
  let result = 0;
  const MAX_INT32 = 2 ** 31 - 1;
  const MIN_INT32 = -MAX_INT32 - 1;
  const isNegative = x < 0;

  if (isNegative) {
    if (x == MIN_INT32) {
      return 0;
    }
    x = -x;
  }

  while (x != 0) {
    const lastDigit = x % 10;
    result = result * 10 + lastDigit;
    if (result > MAX_INT32 || result < MIN_INT32) {
      return 0;
    }
    x = Math.floor(x / 10);
  }

  return isNegative ? -result : result;
}

export { reverse };
