import romanToInt from './13';
import { describe, test, expect } from 'bun:test';

interface tt {
  s: string;
  want: number;
}

const tests: tt[] = [
  { s: 'III', want: 3 },
  { s: 'LVIII', want: 58 },
  { s: 'MCMXCIV', want: 1994 },
];

describe('roman to integer', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = romanToInt(t.s);
      expect(result).toBe(t.want);
    });
  });
});
