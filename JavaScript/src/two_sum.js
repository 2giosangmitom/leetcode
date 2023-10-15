import { it, describe } from "node:test";
import assert from "node:assert";

/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number[]}
 */
function twoSum(nums, target) {
  const map = new Map();
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    if (map.has(complement)) {
      return [map.get(complement), i];
    }
    map.set(nums[i], i);
  }
  return [-1];
}

/**
 * @param {number[]} nums
 * @param {number} target
 * @returns {number[]}
 */
function twoSum2(nums, target) {
  for (let i = 0; i < nums.length; i++) {
    const complement = target - nums[i];
    for (let j = i + 1; j < nums.length; j++) {
      if (nums[j] === complement) {
        return [i, j];
      }
    }
  }
  return [-1];
}

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
    it(JSON.stringify(tt), () => {
      assert.deepStrictEqual(twoSum(tt.nums, tt.target), tt.want);
      assert.deepStrictEqual(twoSum2(tt.nums, tt.target), tt.want);
    });
  }
});
