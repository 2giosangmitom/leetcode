function romanToInt(s: string): number {
  let result = 0;
  let number = 0;
  for (let i = s.length - 1; i >= 0; --i) {
    switch (s[i]) {
      case "I":
        number = 1;
        break;
      case "V":
        number = 5;
        break;
      case "X":
        number = 10;
        break;
      case "L":
        number = 50;
        break;
      case "C":
        number = 100;
        break;
      case "D":
        number = 500;
        break;
      case "M":
        number = 1000;
        break;
    }
    if (number * 4 < result) {
      result -= number;
    } else {
      result += number;
    }
  }
  return result;
}

export default romanToInt;
