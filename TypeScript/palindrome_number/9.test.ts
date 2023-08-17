import { assertEquals } from "https://deno.land/std@0.198.0/assert/mod.ts";
import isPalindrome from "./9.ts";

interface tt {
  num: number;
  want: boolean;
}

Deno.test("palindrome number", () => {
  const tests: tt[] = [
    { num: 121, want: true },
    { num: -321, want: false },
    { num: 10, want: false },
  ];

  for (const t of tests) {
    const result = isPalindrome(t.num);
    assertEquals(result, t.want);
  }
});
