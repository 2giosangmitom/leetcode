// Valid Parentheses
const isValid = function (s) {
	const openers = new Set(["{", "[", "("]);
	const closers = new Set(["}", "]", ")"]);
	const map = new Map();

	map.set("{", "}");
	map.set("(", ")");
	map.set("[", "]");

	const stack = [];

	for (const bracket of s) {
		if (openers.has(bracket)) {
			stack.push(bracket);
		} else if (closers.has(bracket)) {
			const fromStack = stack.pop();
			if (map.get(fromStack) !== bracket) {
				return false;
			}
		}
	}
	return stack.length === 0;
};
