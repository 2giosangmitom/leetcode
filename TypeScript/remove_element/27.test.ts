import { assertEquals } from "../deps.ts";
import removeElement from "./27.ts";

interface tt {
  nums: number[];
  val: number;
  want: number;
}

const tests: tt[] = [
  { nums: [3, 2, 2, 3], val: 3, want: 2 },
  { nums: [0, 1, 2, 2, 3, 0, 4, 2], val: 2, want: 5 },
];

Deno.test("remove element", () => {
  for (const t of tests) {
    const result = removeElement(t.nums, t.val);
    assertEquals(result, t.want);
  }
});
