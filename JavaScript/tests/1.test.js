import { describe, test } from "node:test";
import { twoSum } from "../src/1.js";
import assert from "node:assert";

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
];

describe("1. Two Sum", () => {
  for (let i = 0; i < cases.length; i++) {
    test(`case ${i + 1}`, () => {
      const result = twoSum(cases[i].nums, cases[i].target);
      assert.deepEqual(result, cases[i].want);
    });
  }
});
