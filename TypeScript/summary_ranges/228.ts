function summaryRanges(nums: number[]): string[] {
  const result: string[] = [];

  for (let i = 0; i < nums.length; i++) {
    let end = i;
    while (end + 1 < nums.length && nums[end + 1] - nums[end] == 1) {
      end++;
    }

    if (end > i) {
      result.push(`${nums[i]}->${nums[end]}`);
    } else {
      result.push(`${nums[end]}`);
    }

    i = end;
  }

  return result;
}

export { summaryRanges };
