import countOdds from './1523';
import { describe, test, expect } from 'bun:test';

interface tt {
  low: number;
  high: number;
  want: number;
}

const tests: tt[] = [
  { low: 8, high: 10, want: 1 },
  { low: 3, high: 6, want: 2 },
  { low: 8, high: 13, want: 3 },
  { low: 3, high: 7, want: 3 },
  { low: 2, high: 6, want: 2 },
];

describe('count odd numbers', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = countOdds(t.low, t.high);
      expect(result).toBe(t.want);
    });
  });
});
