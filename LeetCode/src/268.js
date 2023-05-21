// Missing Number
const missingNumber = function (nums) {
	const total = (nums.length * (nums.length + 1)) / 2;
	let sum = 0;
	nums.forEach((value) => {
		sum += value;
	});
	return total - sum;
};
