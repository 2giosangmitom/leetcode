// Two Sum
function twoSum(nums: number[], target: number): number[] | number {
  const hashMap = new Map<number, number>();
  for (let i = 0; i < nums.length; ++i) {
    const need = target - nums[i];
    if (hashMap.has(need)) {
      return [hashMap.get(need), i];
    }
    hashMap.set(nums[i], i);
  }
  return -1;
}

console.log(twoSum([2, 7, 11, 15], 9));
