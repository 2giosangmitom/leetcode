/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[]}
 */
function twoSum(nums, target) {
  const hashMap = new Map();
  for (let i = 0; i < nums.length; i++) {
    const remainder = target - nums[i];
    if (hashMap.get(remainder) === undefined) {
      hashMap.set(nums[i], i);
    } else {
      return [hashMap.get(remainder), i];
    }
  }
  return [];
}

export { twoSum };
