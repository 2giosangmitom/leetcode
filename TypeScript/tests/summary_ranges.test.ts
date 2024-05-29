import { assertEquals } from "@std/assert";
import { summaryRanges } from "../src/summary_ranges.ts";

const cases = [
  { nums: [0, 1, 2, 4, 5, 7], want: ["0->2", "4->5", "7"] },
  { nums: [0, 2, 3, 4, 6, 8, 9], want: ["0", "2->4", "6", "8->9"] },
];

for (let i = 0; i < cases.length; i++) {
  const t = cases[i];
  Deno.test(`summaryRanges - test case ${i + 1}`, () => {
    const result = summaryRanges(t.nums);
    assertEquals(result, t.want);
  });
}
