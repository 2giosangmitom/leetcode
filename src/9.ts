function isPalindrome(x: number): boolean {
  if (x < 0) return false;
  if (x <= 9 && x >= 0) return true;
  return +x.toString().split("").reverse().join("") === x;
}

export default isPalindrome;
