import assert from "node:assert";
import { describe, test } from "node:test";
import { reverse } from "../src/7.js";

const cases = [
  { x: 123, want: 321 },
  { x: -123, want: -321 },
  { x: 120, want: 21 },
  { x: 1463847412, want: 2147483641 },
  { x: -2147483648, want: 0 },
];

describe("7. Reverse Integer", () => {
  for (let i = 0; i < cases.length; i++) {
    test(`case ${i + 1}`, () => {
      const result = reverse(cases[i].x);
      assert.equal(result, cases[i].want);
    });
  }
});
