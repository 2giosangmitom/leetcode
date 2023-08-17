import { assertEquals } from "https://deno.land/std@0.198.0/assert/mod.ts";
import twoSum from "./1.ts";

interface tt {
  nums: number[];
  target: number;
  want: number[];
}

Deno.test("two sum", () => {
  const tests: tt[] = [
    {
      nums: [2, 7, 11, 15],
      target: 9,
      want: [0, 1],
    },
    {
      nums: [3, 2, 4],
      target: 6,
      want: [1, 2],
    },
    {
      nums: [2, 3, 4, 1, 25, 8],
      target: 30,
      want: [-1],
    },
  ];

  for (const t of tests) {
    const result = twoSum(t.nums, t.target);
    assertEquals(result, t.want);
  }
});
