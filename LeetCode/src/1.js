// Two Sum
const twoSum = function (nums, target) {
	const hash = {};
	for (let i = 0; i < nums.length; i++) {
		const need = target - nums[i];
		if (hash[need] !== undefined) {
			return [hash[need], i];
		}
		hash[nums[i]] = i;
	}
	return -1;
};
