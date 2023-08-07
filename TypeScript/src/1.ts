function twoSum(nums: number[], target: number): number[] {
  const hashMap: Map<number, number> = new Map();
  for (let i = 0; i < nums.length; ++i) {
    const needNumber: number = target - nums[i];
    if (hashMap.has(needNumber)) {
      return [hashMap.get(needNumber) ?? 0, i];
    } else {
      hashMap.set(nums[i], i);
    }
  }
  return [-1];
}

export default twoSum;
