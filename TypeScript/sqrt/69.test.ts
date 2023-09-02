import mySqrt from "./69";

interface tt {
  x: number;
  want: number;
}

const tests: tt[] = [
  { x: 4, want: 2 },
  { x: 8, want: 2 },
];

describe("sqrt", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = mySqrt(t.x);
      expect(result).toBe(t.want);
    });
  });
});
