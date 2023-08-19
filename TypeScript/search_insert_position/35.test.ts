import { assertEquals } from "../deps.ts";
import searchInsert from "./35.ts";

interface tt {
  nums: number[];
  target: number;
  want: number;
}

const tests: tt[] = [
  { nums: [1, 3, 5, 6], target: 5, want: 2 },
  { nums: [1, 3, 5, 6], target: 2, want: 1 },
  { nums: [1, 3, 5, 6], target: 7, want: 4 },
];

Deno.test("search insert position", () => {
  for (const t of tests) {
    const result = searchInsert(t.nums, t.target);
    assertEquals(result, t.want);
  }
});
