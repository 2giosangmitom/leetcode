import { assertEquals } from "@std/assert";
import { reverse } from "../src/reverse_integer.ts";

const cases = [
  { x: 123, want: 321 },
  { x: -123, want: -321 },
  { x: 120, want: 21 },
  { x: 1534236469, want: 0 },
  { x: -2147483648, want: 0 },
  { x: 900000, want: 9 },
];

for (let i = 0; i < cases.length; i++) {
  const t = cases[i];
  Deno.test(`reverseInteger - test case ${i + 1}`, () => {
    const result = reverse(t.x);
    assertEquals(result, t.want);
  });
}
