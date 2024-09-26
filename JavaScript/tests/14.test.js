import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { longestCommonPrefix } from "../src/14.js";

const cases = [
  { strs: ["flower", "flow", "flight"], want: "fl" },
  { strs: ["dog", "racecar", "car"], want: "" },
];

describe("14. Longest Common Prefix", () => {
  for (let i = 0; i < cases.length; i++) {
    test(`case ${i + 1}`, () => {
      const result = longestCommonPrefix(cases[i].strs);
      assert.equal(result, cases[i].want);
    });
  }
});
