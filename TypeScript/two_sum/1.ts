/**
 * Runtime: 59ms (Beats 89.87%)
 * Memory: 44.9MB (Beats 56.21%)
 */

function twoSum(nums: number[], target: number): number[] {
  const hashMap = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const needNumber = target - nums[i];
    if (hashMap.has(needNumber)) {
      return [hashMap.get(needNumber) ?? 0, i];
    } else {
      hashMap.set(nums[i], i);
    }
  }
  return [-1];
}

export default twoSum;
