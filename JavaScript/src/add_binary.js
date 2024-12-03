/**
 * @param {string} s
 * @returns {string} the string with no leading zeros
 */
function trimLeadingZeros(s) {
  const firstOne = s.indexOf("1");
  return firstOne === -1 ? "0" : s.slice(firstOne);
}

/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
function addBinary(a, b) {
  let result = "";
  a = trimLeadingZeros(a);
  b = trimLeadingZeros(b);

  if (a.length < b.length) {
    return addBinary(b, a);
  }

  let j = b.length - 1;
  let carry = 0;

  for (let i = a.length - 1; i >= 0; i--) {
    let sum = +a[i] + carry;

    if (j >= 0) {
      sum += +b[j];
      j--;
    }

    const bit = sum % 2;
    carry = Math.floor(sum / 2);
    result = bit.toString().concat(result);
  }

  if (carry > 0) {
    result = "1".concat(result);
  }

  return result;
}

export { addBinary };
