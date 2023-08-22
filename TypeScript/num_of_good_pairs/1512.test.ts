import { assertEquals } from "../deps.ts";
import numIdenticalPairs from "./1512.ts";

interface tt {
  nums: number[];
  want: number;
}

const cases: tt[] = [
  { nums: [1, 2, 3, 1, 1, 3], want: 4 },
  { nums: [1, 1, 1, 1], want: 6 },
  { nums: [1, 2, 3], want: 0 },
];

Deno.test("number of good pairs", () => {
  for (const t of cases) {
    const result = numIdenticalPairs(t.nums);
    assertEquals(result, t.want);
  }
});
