function strStr(haystack: string, needle: string): number {
  const haystackLen = haystack.length;
  const needleLen = needle.length;
  if (needleLen > haystackLen) return -1;
  for (let i = 0; i <= haystackLen - needleLen; i++) {
    const s = haystack.slice(i, i + needleLen);
    if (s === needle) {
      return i;
    }
  }
  return -1;
}

export default strStr;
