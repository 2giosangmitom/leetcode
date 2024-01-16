function twoSum(nums: number[], target: number): number[] {
	const map = new Map<number, number>();
	for (let i = 0; i < nums.length; i++) {
		const complement = target - nums[i];
		const value = map.get(complement);
		if (value === undefined) {
			map.set(nums[i], i);
		} else {
			return [value, i];
		}
	}
	return [-1];
}

export { twoSum };
