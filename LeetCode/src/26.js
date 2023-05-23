// Remove Duplicates from Sorted Array
const removeDuplicates = function (nums) {
	let j = 1;
	for (let i = 1; i < nums.length; ++i) {
		if (nums[i] !== nums[i - 1]) {
			nums[j] = nums[i];
			++j;
		}
	}
	return j;
};
