import { assertEquals } from "../deps.ts";
import search from "./704.ts";

interface tt {
  nums: number[];
  target: number;
  want: number;
}

const tests: tt[] = [
  { nums: [-1, 0, 3, 5, 9, 12], target: 9, want: 4 },
  { nums: [-1, 0, 3, 5, 9, 12], target: 2, want: -1 },
];

Deno.test("binary search", () => {
  for (const t of tests) {
    const result = search(t.nums, t.target);
    assertEquals(result, t.want);
  }
});
