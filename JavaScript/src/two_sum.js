/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number[]}
 */
function twoSum(nums, target) {
  const map = new Map();
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    if (map.has(complement)) {
      return [map.get(complement), i];
    }
    map.set(nums[i], i);
  }
  return [-1];
}

/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number[]}
 */
function twoSum2(nums, target) {
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    for (let j = i + 1; j < nums.length; j++) {
      if (nums[j] === complement) {
        return [i, j];
      }
    }
  }
  return [-1];
}

export { twoSum, twoSum2 };
