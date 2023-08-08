import twoSum from "../src/1";
import { describe, expect, test } from "@jest/globals";

interface tt {
  nums: number[];
  target: number;
  expect: number[];
}

const cases: tt[] = [
  { nums: [2, 7, 11, 15], target: 9, expect: [0, 1] },
  { nums: [3, 2, 4], target: 6, expect: [1, 2] },
  { nums: [3, 3], target: 6, expect: [0, 1] },
  { nums: [2, 3, 4, 1, 25, 8], target: 30, expect: [-1] },
];

describe("Two Sum", () => {
  for (const [i, t] of cases.entries()) {
    test(`case ${i}`, () => {
      const got = twoSum(t.nums, t.target);
      expect(got).toEqual(t.expect);
    });
  }
});
