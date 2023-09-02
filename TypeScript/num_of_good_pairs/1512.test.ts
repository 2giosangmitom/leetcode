import numIdenticalPairs from "./1512";

interface tt {
  nums: number[];
  want: number;
}

const tests: tt[] = [
  { nums: [1, 2, 3, 1, 1, 3], want: 4 },
  { nums: [1, 1, 1, 1], want: 6 },
  { nums: [1, 2, 3], want: 0 },
];

describe("number of good pairs", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = numIdenticalPairs(t.nums);
      expect(result).toBe(t.want);
    });
  });
});
