import twoSum from './1';
import { describe, test, expect } from 'bun:test';

interface tt {
  nums: number[];
  target: number;
  want: number[];
}

const tests: tt[] = [
  {
    nums: [2, 7, 11, 15],
    target: 9,
    want: [0, 1],
  },
  {
    nums: [3, 2, 4],
    target: 6,
    want: [1, 2],
  },
  {
    nums: [2, 3, 4, 1, 25, 8],
    target: 30,
    want: [-1],
  },
];

describe('two sum', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = twoSum(t.nums, t.target);
      expect(result).toEqual(t.want);
    });
  });
});
