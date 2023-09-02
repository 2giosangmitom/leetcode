function missingNumber(nums: number[]): number {
  const sum = nums.reduce((prev, curr) => (prev += curr), 0);
  const total = (nums.length * (nums.length + 1)) / 2;
  return total - sum;
}

export default missingNumber;
