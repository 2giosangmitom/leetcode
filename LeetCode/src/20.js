// Valid Parentheses
const isValid = function (s) {
	if (s.length % 2 !== 0) {
		return false;
	}
	const stack = [];
	for (char of s) {
		if (char === "(" || char === "{" || char === "[") {
			stack.push(char);
		} else {
			if (stack.length === 0) {
				return false;
			} else if (char === ")" && stack[stack.length - 1] === "(") {
				stack.pop();
			} else if (char === "}" && stack[stack.length - 1] === "{") {
				stack.pop();
			} else if (char === "]" && stack[stack.length - 1] === "[") {
				stack.pop();
			} else {
				return false;
			}
		}
	}
	return stack.length === 0;
};
