import { assertEquals } from "../deps.ts";
import missingNumber from "./268.ts";

interface tt {
  nums: number[];
  want: number;
}

const cases: tt[] = [
  { nums: [3, 0, 1], want: 2 },
  { nums: [0, 1], want: 2 },
  { nums: [9, 6, 4, 2, 3, 5, 7, 0, 1], want: 8 },
];

for (const t of cases) {
  Deno.test("missing number", () => {
    const result = missingNumber(t.nums);
    assertEquals(result, t.want);
  });
}
