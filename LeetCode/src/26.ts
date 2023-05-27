// Remove Duplicates from Sorted Array
function removeDuplicates(nums: number[]): number {
	let j = 1;
	for (let i = 1; i < nums.length; ++i) {
		if (nums[i] !== nums[i - 1]) {
			nums[j] = nums[i];
			++j;
		}
	}
	return j;
}
