import isPalindrome from './9';
import { describe, test, expect } from 'bun:test';

interface tt {
  num: number;
  want: boolean;
}

const tests: tt[] = [
  { num: 121, want: true },
  { num: -321, want: false },
  { num: 10, want: false },
];

describe('palindrome number', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = isPalindrome(t.num);
      expect(result).toBe(t.want);
    });
  });
});
