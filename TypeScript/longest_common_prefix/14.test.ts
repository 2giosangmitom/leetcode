import longestCommonPrefix from './14';
import { describe, test, expect } from 'bun:test';

interface tt {
  strs: string[];
  want: string;
}

const tests: tt[] = [
  {
    strs: ['flower', 'flow', 'flight'],
    want: 'fl',
  },
  {
    strs: ['dog', 'racecar', 'car'],
    want: '',
  },
];

describe('longest common prefix', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = longestCommonPrefix(t.strs);
      expect(result).toBe(t.want);
    });
  });
});
