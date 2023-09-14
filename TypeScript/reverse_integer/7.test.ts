import reverse from './7';
import { describe, test, expect } from 'bun:test';

interface tt {
  num: number;
  want: number;
}

const tests: tt[] = [
  { num: 123, want: 321 },
  { num: 900000, want: 9 },
  { num: -123, want: -321 },
  { num: 553126124224, want: 0 },
];

describe('reverse integer', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = reverse(t.num);
      expect(result).toBe(t.want);
    });
  });
});
