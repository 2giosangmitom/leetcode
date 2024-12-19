/**
 * @param {string[]} strs
 * @return {string[][]}
 */
function groupAnagrams(strs) {
  /**@type {Map<string, string[]>} */
  const map = new Map();

  for (const s of strs) {
    const key = s.split("").sort().join("");
    const group = map.get(key);
    if (group !== undefined) {
      group.push(s);
    } else {
      const newGroup = [s];
      map.set(key, newGroup);
    }
  }

  /** @type {string[][]} */
  const result = [];
  map.forEach((v) => {
    result.push(v);
  });

  return result;
}

export { groupAnagrams };
