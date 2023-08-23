import { assertEquals } from "../deps.ts";
import isPalindrome from "./9.ts";

interface tt {
  num: number;
  want: boolean;
}

const tests: tt[] = [
  { num: 121, want: true },
  { num: -321, want: false },
  { num: 10, want: false },
];

for (const t of tests) {
  Deno.test("palindrome number", () => {
    const result = isPalindrome(t.num);
    assertEquals(result, t.want);
  });
}
