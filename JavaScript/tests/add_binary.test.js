import { expect, test } from "vitest";
import { addBinary } from "#src/add_binary";

const cases = [
  { a: "11", b: "1", want: "100" },
  { a: "1010", b: "1011", want: "10101" },
  { a: "0", b: "0", want: "0" },
];

cases.forEach(({ a, b, want }, i) => {
  test(`case ${i + 1}`, () => {
    const actual = addBinary(a, b);
    expect(actual).toEqual(want);
  });
});
