import { assertEquals } from "@std/assert";
import { singleNumber } from "../src/single_number.ts";

const cases = [
  { nums: [2, 2, 1], want: 1 },
  { nums: [4, 1, 2, 1, 2], want: 4 },
  { nums: [1], want: 1 },
];

for (let i = 0; i < cases.length; i++) {
  const t = cases[i];
  Deno.test(`singleNumber - test case ${i + 1}`, () => {
    const result = singleNumber(t.nums);
    assertEquals(result, t.want);
  });
}
