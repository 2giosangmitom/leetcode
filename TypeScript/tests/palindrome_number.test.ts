import { isPalindrome, isPalindrome2 } from "@/palindrome_number";
import { describe, test, expect } from "bun:test";

describe("palindrome number", () => {
  const cases = [
    { num: -10, want: false },
    { num: 5, want: true },
    { num: 121, want: true },
    { num: 321, want: false },
    { num: 111, want: true },
  ];

  for (const tt of cases) {
    test(JSON.stringify(tt), () => {
      expect(isPalindrome(tt.num)).toBe(tt.want);
      expect(isPalindrome2(tt.num)).toBe(tt.want);
    });
  }
});
