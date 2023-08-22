/**
 * Runtime: 53ms (Beats 80.25%)
 * Memory: 42.10MB (Beats 97.54%)
 */

function lengthOfLastWord(s: string): number {
  let result = 0;
  s = s.trim();
  for (let i = s.length - 1; i >= 0 && s[i] !== " "; --i) {
    result++;
  }
  return result;
}

export default lengthOfLastWord;
