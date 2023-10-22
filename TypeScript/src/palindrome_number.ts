function isPalindrome(x: number): boolean {
  if (x < 0) return false;

  const reverse = (x: number): number => {
    let result = 0;

    while (x != 0) {
      const lastDigit = x % 10;
      result = result * 10 + lastDigit;
      x = Math.floor(x / 10);
    }

    return result;
  };

  return reverse(x) === x;
}

// NOTE: TypeScript hack
function isPalindrome2(x: number): boolean {
  return +x.toString().split("").reverse().join("") == x;
}

export { isPalindrome, isPalindrome2 };
