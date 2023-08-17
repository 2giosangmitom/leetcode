/**
 * Runtime: 122ms (Beats 98.85%)
 * Memory: 50.9MB (Beats 96.45%)
 */

function isPalindrome(x: number): boolean {
  if (x < 0) return false;

  const reverve = function (num: number): number {
    let result = 0;
    while (num != 0) {
      const lastNumber = num % 10;
      result = result * 10 + lastNumber;
      num = Math.floor(num / 10);
    }
    return result;
  };

  return reverve(x) === x;
}

export default isPalindrome;
