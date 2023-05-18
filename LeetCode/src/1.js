// Two Sum
const twoSum = function (nums, target) {
	for (let i = 0; i < nums.length; ++i) {
		const need = target - nums[i];
		for (let j = 0; j < nums.length; ++j) {
			if (nums[j] === need && j !== i) {
				return [i, j];
			}
		}
	}
	return -1;
};
