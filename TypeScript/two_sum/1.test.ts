import { assertEquals } from "../deps.ts";
import { twoSum, twoSum2 } from "./1.ts";

interface tt {
  nums: number[];
  target: number;
  want: number[];
}

const cases: tt[] = [
  { nums: [2, 7, 11, 15], target: 9, want: [0, 1] },
  { nums: [3, 2, 4], target: 6, want: [1, 2] },
  { nums: [3, 3], target: 6, want: [0, 1] },
  { nums: [2, 3, 4, 1, 25, 8], target: 30, want: [-1] },
];

for (const t of cases) {
  Deno.test(`twoSum(${JSON.stringify(t.nums)}, ${t.target})`, () => {
    const result = twoSum(t.nums, t.target);
    assertEquals(result, t.want);
  });

  Deno.test(`twoSum2(${JSON.stringify(t.nums)}, ${t.target})`, () => {
    const result = twoSum2(t.nums, t.target);
    assertEquals(result, t.want);
  });
}
