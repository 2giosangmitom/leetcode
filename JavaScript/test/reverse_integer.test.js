import reverse from "../src/reverse_integer.js";
import { test, describe } from "node:test";
import assert from "node:assert";

describe("reverse integer", () => {
  const cases = [
    { x: 123, want: 321 },
    { x: -123, want: -321 },
    { x: 120, want: 21 },
    { x: 1534236469, want: 0 },
    { x: -2147483648, want: 0 },
    { x: 900000, want: 9 },
  ];

  for (const tt of cases) {
    test(JSON.stringify(tt), () => {
      assert.equal(reverse(tt.x), tt.want);
    });
  }
});
