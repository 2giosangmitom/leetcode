import maxProfit from './121';
import { describe, test, expect } from 'bun:test';

interface tt {
  prices: number[];
  want: number;
}

const tests: tt[] = [
  { prices: [7, 1, 5, 3, 6, 4], want: 5 },
  { prices: [7, 6, 4, 3, 1], want: 0 },
];

describe('best time to buy and sell stock', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = maxProfit(t.prices);
      expect(result).toBe(t.want);
    });
  });
});
