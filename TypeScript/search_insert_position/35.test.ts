import searchInsert from './35';
import { describe, test, expect } from 'bun:test';

interface tt {
  nums: number[];
  target: number;
  want: number;
}

const tests: tt[] = [
  { nums: [1, 3, 5, 6], target: 5, want: 2 },
  { nums: [1, 3, 5, 6], target: 2, want: 1 },
  { nums: [1, 3, 5, 6], target: 7, want: 4 },
];

describe('search insert position', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = searchInsert(t.nums, t.target);
      expect(result).toBe(t.want);
    });
  });
});
