import numWaterBottles from './1518';
import { describe, test, expect } from 'bun:test';

interface tt {
  numBottles: number;
  numExchange: number;
  want: number;
}

const tests: tt[] = [
  { numBottles: 9, numExchange: 3, want: 13 },
  { numBottles: 15, numExchange: 4, want: 19 },
];

describe('water bottles', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = numWaterBottles(t.numBottles, t.numExchange);
      expect(result).toBe(t.want);
    });
  });
});
