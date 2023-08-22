/**
 * Runtime: 74ms (Beats 57.66%)
 * Memory: 44.69MB (Beats 59.86%)
 */

function mySqrt(x: number): number {
  let left = 1;
  let right = x;
  while (left <= right) {
    const mid = Math.floor(left + (right - left) / 2);
    if (mid === Math.floor(x / mid)) {
      return mid;
    } else if (mid > Math.floor(x / mid)) {
      right = mid - 1;
    } else {
      left = mid + 1;
    }
  }
  return left - 1;
}

export default mySqrt;
