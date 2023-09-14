function removeDuplicates(nums: number[]): number {
  let k = 0;
  for (let i = 1; i < nums.length; i++) {
    if (nums[k] !== nums[i]) {
      k++;
      nums[k] = nums[i];
    }
  }
  return k + 1;
}

export default removeDuplicates;
