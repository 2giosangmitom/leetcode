import { sortArray } from "../src/sort_an_array.js";
import { assertEquals } from "@std/assert";

const cases = [
  {
    name: "reversed array",
    nums: [7, 6, 5, 4, 3, 2, 1],
    want: [1, 2, 3, 4, 5, 6, 7],
  },
  {
    name: "random order",
    nums: [5, 1, 1, 2, 0, 0, 6, 63, 12, 4],
    want: [0, 0, 1, 1, 2, 4, 5, 6, 12, 63],
  },
];

for (const tt of cases) {
  Deno.test(tt.name, () => {
    assertEquals(sortArray(tt.nums), tt.want);
  });
}
