function lengthOfLastWord(s: string): number {
  let result = 0;
  s = s.trim();
  for (let i = s.length - 1; i >= 0 && s[i] !== ' '; --i) {
    result++;
  }
  return result;
}

export default lengthOfLastWord;
