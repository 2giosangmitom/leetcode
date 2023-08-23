import { assertEquals } from "../deps.ts";
import removeDuplicates from "./26.ts";

interface tt {
  nums: number[];
  want: number;
}

const tests: tt[] = [
  { nums: [1, 1, 2], want: 2 },
  { nums: [0, 0, 1, 1, 1, 2, 2, 3, 3, 4], want: 5 },
];

for (const t of tests) {
  Deno.test("remove duplicates from sorted array", () => {
    const result = removeDuplicates(t.nums);
    assertEquals(result, t.want);
  });
}
