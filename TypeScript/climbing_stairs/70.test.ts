import { describe, expect, test } from 'bun:test';
import climbStairs from './70';

interface tt {
  n: number;
  want: number;
}

const tests: tt[] = [
  { n: 2, want: 2 },
  { n: 3, want: 3 },
  { n: 4, want: 5 },
];

describe('climbing stairs', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = climbStairs(t.n);
      expect(result).toBe(t.want);
    });
  });
});
