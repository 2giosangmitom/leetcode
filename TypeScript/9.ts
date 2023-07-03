function isPalindrome(x: number): boolean {
  if (x < 0) return false;
  if (x <= 9 && x >= 0) return true;
  const reverseNumber = x.toString().split("").reverse().join("");
  return Number(reverseNumber) === x;
}

console.log(isPalindrome(121)); // TEST: => true
console.log(isPalindrome(-121)); // TEST: => false
