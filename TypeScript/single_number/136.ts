function singleNumber(nums: number[]): number {
  const map = new Map<number, number>();

  nums.forEach((num) => {
    const v = map.get(num);
    if (v == undefined) {
      map.set(num, 1);
    } else {
      map.set(num, v + 1);
    }
  });

  for (let i = 0; i < nums.length; i++) {
    const v = map.get(nums[i]);
    if (v === 1) {
      return nums[i];
    }
  }

  return -1;
}

export { singleNumber };
