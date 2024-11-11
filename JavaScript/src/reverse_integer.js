/**
 * @param {number} x
 * @return {number}
 */
function reverse(x) {
  let result = 0;
  const MAX_INT32 = 2 ** 31 - 1;
  const MIN_INT32 = -MAX_INT32 - 1;
  const isNegative = x < 0;
  let temp = x;

  if (isNegative) {
    if (temp === MIN_INT32) {
      return 0;
    }
    temp = -temp;
  }

  while (temp !== 0) {
    const lastDigit = temp % 10;
    result = result * 10 + lastDigit;
    if (result > MAX_INT32 || result < MIN_INT32) {
      return 0;
    }
    temp = Math.floor(temp / 10);
  }

  return isNegative ? -result : result;
}

export { reverse };
