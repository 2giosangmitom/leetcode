/**
 * @param {number[]} nums
 * @param {number} p
 * @param {number} q
 * @param {number} m
 */
function merge(nums, p, q, m) {
  const left = nums.slice(p, m + 1);
  const right = nums.slice(m + 1, q + 1);

  let i = 0;
  let j = 0;
  let k = p;

  while (i < left.length && j < right.length) {
    if (left[i] < right[j]) {
      nums[k] = left[i];
      i++;
    } else {
      nums[k] = right[j];
      j++;
    }
    k++;
  }

  while (i < left.length) {
    nums[k] = left[i];
    i++;
    k++;
  }

  while (j < right.length) {
    nums[k] = right[j];
    j++;
    k++;
  }
}

/**
 * @param {number[]} nums
 * @param {number} p
 * @param {number} q
 */
function mergeSort(nums, p, q) {
  if (p < q) {
    const m = Math.floor(p + (q - p) / 2);
    mergeSort(nums, p, m);
    mergeSort(nums, m + 1, q);
    merge(nums, p, q, m);
  }
}

/**
 * @param {number[]} nums
 * @return {number[]}
 */
function sortArray(nums) {
  mergeSort(nums, 0, nums.length - 1);
  return nums;
}

export { sortArray };
