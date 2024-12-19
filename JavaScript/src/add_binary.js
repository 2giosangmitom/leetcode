/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
function addBinary(a, b) {
  const len_a = a.length;
  const len_b = b.length;

  let i = len_a - 1;
  let j = len_b - 1;
  let carry = 0;

  let result = "";
  while (j >= 0 || j >= 0 || carry > 0) {
    let bit1 = i >= 0 ? Number(a[i--]) : 0;
    let bit2 = j >= 0 ? Number(b[j--]) : 0;
    let sum = bit1 + bit2 + carry;
    result += (sum % 2).toString();
    carry = Math.floor(sum / 2);
  }

  return result.split("").reverse().join("");
}

export { addBinary };
