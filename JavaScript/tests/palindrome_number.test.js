import { expect, test } from "vitest";
import { isPalindrome } from "../src/palindrome_number.js";

const cases = [
  {
    name: "positive palindrome",
    x: 121,
    want: true,
  },
  {
    name: "negative palindrome",
    x: -121,
    want: false,
  },
  {
    name: "not palindrome ending in zero",
    x: 10,
    want: false,
  },
  {
    name: "not palindrome large number",
    x: 1234567899,
    want: false,
  },
];

for (const tt of cases) {
  test(tt.name, () => {
    const actual = isPalindrome(tt.x);
    expect(actual).toBe(tt.want);
  });
}
