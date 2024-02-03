function isPalindrome(x: number): boolean {
  if (x < 0) {
    return false;
  }

  const reverse = (num: number) => {
    let result = 0;

    while (num !== 0) {
      const lastDigit = num % 10;
      result = result * 10 + lastDigit;
      num = Math.floor(num / 10);
    }

    return result;
  };

  return reverse(x) === x;
}

export { isPalindrome };
