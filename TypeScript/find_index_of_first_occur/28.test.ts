import strStr from './28';
import { describe, test, expect } from 'bun:test';

interface tt {
  haystack: string;
  needle: string;
  want: number;
}

const tests: tt[] = [
  { haystack: 'sadbutsad', needle: 'sad', want: 0 },
  { haystack: 'leetcode', needle: 'leeto', want: -1 },
  { haystack: 'hello', needle: 'll', want: 2 },
  { haystack: 'a', needle: 'a', want: 0 },
  { haystack: 'abc', needle: 'c', want: 2 },
  { haystack: 'baaa', needle: 'aaaa', want: -1 },
];

describe('find index of the first occur string', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = strStr(t.haystack, t.needle);
      expect(result).toBe(t.want);
    });
  });
});
