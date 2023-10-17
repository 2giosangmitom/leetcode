import { test, describe } from "node:test";
import assert from "node:assert";
import { twoSum, twoSum2 } from "../src/two_sum.js";

describe("two sum", () => {
  const cases = [
    {
      nums: [2, 7, 11, 15],
      target: 9,
      want: [0, 1],
    },
    {
      nums: [3, 2, 4],
      target: 6,
      want: [1, 2],
    },
    {
      nums: [3, 3],
      target: 6,
      want: [0, 1],
    },
    {
      nums: [2, 3, 4, 1, 25, 8],
      target: 30,
      want: [-1],
    },
  ];

  for (const tt of cases) {
    test(JSON.stringify(tt), () => {
      assert.deepStrictEqual(twoSum(tt.nums, tt.target), tt.want);
      assert.deepStrictEqual(twoSum2(tt.nums, tt.target), tt.want);
    });
  }
});
