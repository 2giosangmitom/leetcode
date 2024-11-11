import { removeElement } from "../src/remove_element.js";
import { assertEquals } from "@std/assert";

const cases = [
  {
    name: "continuous occurrences",
    nums: [3, 2, 2, 3],
    val: 3,
    want: 2,
    wantNums: [2, 2],
  },
  {
    name: "multiple occurrences",
    nums: [0, 1, 2, 2, 3, 0, 4, 2],
    val: 2,
    want: 5,
    wantNums: [0, 1, 3, 0, 4],
  },
  {
    name: "no occurrences",
    nums: [1, 2, 3, 4, 5],
    val: 6,
    want: 5,
    wantNums: [1, 2, 3, 4, 5],
  },
  {
    name: "all occurrences",
    nums: [2, 2, 2, 2],
    val: 2,
    want: 0,
    wantNums: [],
  },
  {
    name: "mixed occurrences",
    nums: [4, 5, 4, 6, 4, 7],
    val: 4,
    want: 3,
    wantNums: [5, 6, 7],
  },
];

for (const tt of cases) {
  Deno.test(tt.name, () => {
    const actual = removeElement(tt.nums, tt.val);
    assertEquals(actual, tt.want);
    assertEquals(tt.nums.slice(0, tt.want), tt.wantNums);
  });
}
