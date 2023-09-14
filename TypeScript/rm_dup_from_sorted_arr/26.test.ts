import removeDuplicates from './26';
import { describe, test, expect } from 'bun:test';

interface tt {
  nums: number[];
  want: number;
  wantNums: number[];
}

const tests: tt[] = [
  { nums: [1, 1, 2], want: 2, wantNums: [1, 2] },
  { nums: [0, 0, 1, 1, 1, 2, 2, 3, 3, 4], want: 5, wantNums: [0, 1, 2, 3, 4] },
];

describe('remove duplicates from sorted array', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = removeDuplicates(t.nums);
      expect(result).toBe(t.want);
      expect(t.nums.slice(0, t.want)).toEqual(t.wantNums);
    });
  });
});
