import { assertEquals } from "@std/assert";
import { isPalindrome } from "../src/palindrome_number.ts";

const cases = [
  { num: -10, want: false },
  { num: 5, want: true },
  { num: 121, want: true },
  { num: 321, want: false },
  { num: 111, want: true },
];

for (let i = 0; i < cases.length; i++) {
  const t = cases[i];
  Deno.test(`palindromeNumber - test case ${i + 1}`, () => {
    const result = isPalindrome(t.num);
    assertEquals(result, t.want);
  });
}
