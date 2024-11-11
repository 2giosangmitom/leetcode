/**
 * @param {number} x
 * @return {boolean}
 */
function isPalindrome(x) {
  if (x < 0) {
    return false;
  }

  const reversed = x.toString().split("").reverse().join("");

  return +reversed === x;
}

export { isPalindrome };
