function isPalindrome(x: number): boolean {
  if (x < 0) return false;

  const reverse = (n: number): number => {
    let result = 0;

    while (n != 0) {
      const lastDigit = n % 10;
      result = result * 10 + lastDigit;
      n = Math.floor(n / 10);
    }

    return result;
  };

  return reverse(x) === x;
}

// HACK: TypeScript hack
function isPalindrome2(x: number): boolean {
  return +x.toString().split("").reverse().join("") == x;
}

export { isPalindrome, isPalindrome2 };
