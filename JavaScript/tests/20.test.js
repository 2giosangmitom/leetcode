import assert from "node:assert/strict";
import { describe, test } from "node:test";
import { isValid } from "../src/20.js";

const cases = [
  { s: "()", want: true },
  { s: "()[]{}", want: true },
  { s: "(]", want: false },
  { s: "([])", want: true },
  { s: "([}}])", want: false },
];

describe("20. Valid Parentheses", () => {
  for (let i = 0; i < cases.length; i++) {
    test(`case ${i + 1}`, () => {
      const result = isValid(cases[i].s);
      assert.equal(result, cases[i].want);
    });
  }
});
