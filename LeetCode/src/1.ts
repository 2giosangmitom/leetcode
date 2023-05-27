function twoSum(nums: number[], target: number): number[] {
	const hash: { [index: number]: number } = {};
	for (let i = 0; i < nums.length; ++i) {
		const need: number = target - nums[i]; // 2
		if (hash[need] !== undefined) {
			return [hash[need], i];
		}
		hash[nums[i]] = i;
	}
	return [-1];
}
