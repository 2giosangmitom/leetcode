function removeElement(nums: number[], val: number): number {
  let k = 0;
  nums.forEach((item) => {
    if (item !== val) {
      let temp = item;
      item = nums[k];
      nums[k] = temp;
      ++k;
    }
  });
  return k;
}

console.log(removeElement([3, 2, 2, 3], 3)); // TEST: 2
