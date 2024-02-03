import { assertEquals } from "../deps.ts";
import { isPalindrome } from "./9.ts";

interface tt {
  num: number;
  want: boolean;
}

const cases: tt[] = [
  { num: -10, want: false },
  { num: 5, want: true },
  { num: 121, want: true },
  { num: 321, want: false },
  { num: 111, want: true },
];

for (const [i, t] of cases.entries()) {
  Deno.test(`case ${i}`, () => {
    const result = isPalindrome(t.num);
    assertEquals(result, t.want);
  });
}
