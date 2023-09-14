import removeElement from './27';
import { describe, test, expect } from 'bun:test';

interface tt {
  nums: number[];
  val: number;
  want: number;
  wantNums: number[];
}

const tests: tt[] = [
  {
    nums: [3, 2, 2, 3],
    val: 3,
    want: 2,
    wantNums: [2, 2],
  },
  {
    nums: [0, 1, 2, 2, 3, 0, 4, 2],
    val: 2,
    want: 5,
    wantNums: [0, 1, 3, 0, 4],
  },
];

describe('remove element', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = removeElement(t.nums, t.val);
      expect(result).toBe(t.want);
      expect(t.nums.slice(0, t.want)).toEqual(t.wantNums);
    });
  });
});
