import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { isPalindrome } from "../src/9.js";

const cases = [
  { x: 121, want: true },
  { x: -121, want: false },
  { x: 10, want: false },
];

describe("9. Palindrome Number", () => {
  for (let i = 0; i < cases.length; i++) {
    test(`case ${i + 1}`, () => {
      assert.equal(isPalindrome(cases[i].x), cases[i].want);
    });
  }
});
