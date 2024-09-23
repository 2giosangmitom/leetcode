/**
 * @param {number} x
 * @return {boolean}
 */
function isPalindrome(x) {
  if (x < 0) {
    return false;
  }

  let reversed = x.toString().split("").reverse().join("");

  return reversed == x;
}

export { isPalindrome }
