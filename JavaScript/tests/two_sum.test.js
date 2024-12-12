import { twoSum } from "#src/two_sum.js";
import { expect, test } from "vitest";

const cases = [
  {
    name: "different numbers",
    nums: [2, 7, 11, 15],
    target: 9,
    want: [0, 1],
  },
  {
    name: "different order",
    nums: [3, 2, 4],
    target: 6,
    want: [1, 2],
  },
  {
    name: "same numbers",
    nums: [3, 3],
    target: 6,
    want: [0, 1],
  },
  {
    name: "no solution",
    nums: [1, 2, 3],
    target: 10,
    want: [],
  },
  {
    name: "empty array",
    nums: [],
    target: 5,
    want: [],
  },
];

for (const tt of cases) {
  test(tt.name, () => {
    const actual = twoSum(tt.nums, tt.target);
    expect(actual).toEqual(tt.want);
  });
}
