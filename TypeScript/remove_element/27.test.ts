import { assertEquals } from "../deps.ts";
import removeElement from "./27.ts";

interface tt {
  nums: number[];
  val: number;
  want: number;
  wantNums: number[];
}

const tests: tt[] = [
  {
    nums: [3, 2, 2, 3],
    val: 3,
    want: 2,
    wantNums: [2, 2],
  },
  {
    nums: [0, 1, 2, 2, 3, 0, 4, 2],
    val: 2,
    want: 5,
    wantNums: [0, 1, 3, 0, 4],
  },
];

for (const t of tests) {
  Deno.test("remove element", () => {
    const result = removeElement(t.nums, t.val);
    assertEquals(result, t.want);
    assertEquals(t.nums.slice(0, t.want), t.wantNums);
  });
}
