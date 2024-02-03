import { assertEquals } from "../deps.ts";
import { singleNumber } from "./136.ts";

interface tt {
  nums: number[];
  want: number;
}

const cases: tt[] = [
  { nums: [2, 2, 1], want: 1 },
  { nums: [4, 1, 2, 1, 2], want: 4 },
  { nums: [1], want: 1 },
];

for (const t of cases) {
  Deno.test(JSON.stringify(t), () => {
    const result = singleNumber(t.nums);
    assertEquals(result, t.want);
  });
}
