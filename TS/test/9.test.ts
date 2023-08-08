import isPalindrome from "../src/9";
import { describe, expect, test } from "@jest/globals";

interface tt {
  num: number;
  expect: boolean;
}

const cases: tt[] = [
  { num: -10, expect: false },
  { num: 121, expect: true },
  { num: 10, expect: false },
  { num: 5, expect: true },
];

describe("Palindrome Number", () => {
  for (const [i, t] of cases.entries()) {
    test(`case ${i}`, () => {
      const got = isPalindrome(t.num);
      expect(got).toBe(t.expect);
    });
  }
});
