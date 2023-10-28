import { romanToInt } from "../src/roman_to_integer.ts";
import { describe, test, expect } from "@jest/globals";

type T = {
  roman: string;
  want: number;
};

describe("two sum", () => {
  const cases: T[] = [
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
