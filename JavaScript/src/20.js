/**
 * @param {string} s
 * @return {boolean}
 */
function isValid(s) {
  if (s.length % 2 !== 0) {
    return false;
  }

  const stack = [];
  const matchingBrackets = {
    "(": ")",
    "{": "}",
    "[": "]",
  };

  for (const char of s) {
    if (matchingBrackets[char]) {
      stack.push(matchingBrackets[char]);
    } else {
      if (stack.pop() !== char) {
        return false;
      }
    }
  }

  return stack.length === 0;
}

export { isValid };
