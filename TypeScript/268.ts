function missingNumber(nums: number[]): number {
  let sum = 0;
  const total = (nums.length * (nums.length + 1)) / 2;
  nums.forEach((value) => {
    sum += value;
  });
  return total - sum;
}
