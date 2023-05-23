// Pow(x, n)
const myPow = function (x, n) {
	if (n === 0) return 1;
	const pow = Math.abs(n);
	let result;
	if (pow % 2 === 0) {
		result = myPow(x * x, pow / 2);
	} else {
		result = myPow(x * x, (pow - 1) / 2) * x;
	}
	return n < 0 ? 1 / result : result;
};
