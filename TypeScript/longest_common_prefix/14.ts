/**
 * Runtime: 48ms (Beats 97.58%)
 * Memory: 44.2MB (Beats 71.63%)
 */

function longestCommonPrefix(strs: string[]): string {
  let prefix = strs[0];
  for (const s of strs.slice(1)) {
    while (s.indexOf(prefix) !== 0) {
      prefix = prefix.substring(0, prefix.length - 1);
      if (prefix.length === 0) return "";
    }
  }
  return prefix;
}

export default longestCommonPrefix;
