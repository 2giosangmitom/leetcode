import { twoSum, twoSum2 } from "../src/two_sum.ts";
import { testCases } from "../tests/two_sum.test.ts";

Deno.bench("Benchmark twoSum", { group: "two_sum" }, () => {
  for (const testCase of testCases) {
    twoSum(testCase.nums, testCase.target);
  }
});

Deno.bench("Benchmark twoSum2", { group: "two_sum" }, () => {
  for (const testCase of testCases) {
    twoSum2(testCase.nums, testCase.target);
  }
});
