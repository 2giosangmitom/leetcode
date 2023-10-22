function romanToInt(s: string): number {
  const romanMap = new Map<string, number>([
    ["I", 1],
    ["V", 5],
    ["X", 10],
    ["L", 50],
    ["C", 100],
    ["D", 500],
    ["M", 1000],
  ]);

  let result = 0;

  for (const char of s.split("").reverse()) {
    const number = romanMap.get(char);

    if (number === undefined) {
      return -1;
    }

    if (number * 4 < result) {
      result -= number;
    } else {
      result += number;
    }
  }

  return result;
}

export { romanToInt };
