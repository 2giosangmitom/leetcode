import { singleNumber, singleNumber2 } from "../src/single_number.js";
import { expect, test } from "vitest";

const cases = [
  { name: "same numbers twice", nums: [2, 2, 1], want: 1 },
  { name: "same numbers interleaved", nums: [4, 1, 2, 1, 2], want: 4 },
  { name: "one element", nums: [1], want: 1 },
];

for (const tt of cases) {
  test(tt.name, () => {
    expect(singleNumber(tt.nums), "bit manipulation").toBe(tt.want);
    expect(singleNumber2(tt.nums), "sort").toBe(tt.want);
  });
}
