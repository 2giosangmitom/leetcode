import search from './704';
import { describe, test, expect } from 'bun:test';

interface tt {
  nums: number[];
  target: number;
  want: number;
}

const tests: tt[] = [
  { nums: [-1, 0, 3, 5, 9, 12], target: 9, want: 4 },
  { nums: [-1, 0, 3, 5, 9, 12], target: 2, want: -1 },
];

describe('binary search', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = search(t.nums, t.target);
      expect(result).toBe(t.want);
    });
  });
});
