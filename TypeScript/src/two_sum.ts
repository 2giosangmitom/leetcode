function twoSum(nums: number[], target: number): number[] {
  const map = new Map<number, number>();
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    const v = map.get(complement);
    if (v !== undefined) {
      return [v, i];
    } else {
      map.set(nums[i], i);
    }
  }
  return [-1];
}

function twoSum2(nums: number[], target: number): number[] {
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
