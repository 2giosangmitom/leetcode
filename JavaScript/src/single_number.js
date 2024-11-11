/**
 * @param {number[]} nums
 * @return {number}
 */
function singleNumber(nums) {
  let result = 0;
  for (let i = 0; i < nums.length; i++) {
    result ^= nums[i];
  }
  return result;
}

/**
 * @param {number[]} nums
 * @return {number}
 */
function singleNumber2(nums) {
  nums.sort((a, b) => a - b);
  for (let i = 1; i < nums.length; i += 2) {
    if (nums[i] !== nums[i - 1]) {
      return nums[i - 1];
    }
  }
  return nums[nums.length - 1];
}

export { singleNumber, singleNumber2 };
