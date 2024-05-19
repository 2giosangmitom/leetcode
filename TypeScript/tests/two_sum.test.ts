import { assertEquals } from "@std/assert";
import { twoSum, twoSum2 } from "../src/two_sum.ts";

interface TestCase {
  nums: number[];
  target: number;
  want: number[];
}

const testCases: TestCase[] = [
  { nums: [2, 7, 11, 15], target: 9, want: [0, 1] },
  { nums: [3, 2, 4], target: 6, want: [1, 2] },
  { nums: [3, 3], target: 6, want: [0, 1] },
  { nums: [2, 3, 4, 1, 25, 8], target: 30, want: [-1] },
];

testCases.forEach((testCase, index) => {
  Deno.test(`twoSum - test case ${index + 1}`, () => {
    const result = twoSum(testCase.nums, testCase.target);
    assertEquals(result, testCase.want);
  });

  Deno.test(`twoSum2 - test case ${index + 1}`, () => {
    const result = twoSum2(testCase.nums, testCase.target);
    assertEquals(result, testCase.want);
  });
});
