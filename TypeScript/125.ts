function isPalindrome(s: string): boolean {
  if (s.length === 0) return true;
  const newStr = s.toLowerCase().replace(/[^a-z0-9]/g, "");
  const reversedStr = newStr.split("").reverse().join("").toLowerCase();
  return newStr === reversedStr;
}
