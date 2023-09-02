import missingNumber from "./268";

interface tt {
  nums: number[];
  want: number;
}

const tests: tt[] = [
  { nums: [3, 0, 1], want: 2 },
  { nums: [0, 1], want: 2 },
  { nums: [9, 6, 4, 2, 3, 5, 7, 0, 1], want: 8 },
];

describe("missing number", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = missingNumber(t.nums);
      expect(result).toBe(t.want);
    });
  });
});
