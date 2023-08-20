/**
 * Runtime: 58ms (Beats 63.9%)
 * Memory: 42.8MB (Beats 64.89%)
 */

function strStr(haystack: string, needle: string): number {
  const haystackLen = haystack.length;
  const needleLen = needle.length;
  if (needleLen > haystackLen) return -1;
  for (let i = 0; i < haystackLen; i++) {
    if (
      haystack[i] === needle[0] &&
      haystack.substring(i, i + needleLen) === needle
    ) {
      return i;
    }
  }
  return -1;
}

export default strStr;
