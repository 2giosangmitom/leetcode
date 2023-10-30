import { romanToInt } from "../src/roman_to_integer.ts";
import { describe, test, expect } from "@jest/globals";

describe("two sum", () => {
  const cases = [
    { roman: "III", want: 3 },
    { roman: "LVIII", want: 58 },
    { roman: "MCMXCIV", want: 1994 },
    { roman: "XXIV", want: 24 },
    { roman: "LLVMR", want: -1 },
  ];

  for (const tt of cases) {
    test(JSON.stringify(tt), () => {
      expect(romanToInt(tt.roman)).toBe(tt.want);
    });
  }
});
