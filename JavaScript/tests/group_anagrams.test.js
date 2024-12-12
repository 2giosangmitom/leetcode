import { groupAnagrams } from "#src/group_anagrams.js";
import { expect, test } from "vitest";

const cases = [
  {
    name: "same length",
    strs: ["eat", "tea", "tan", "ate", "nat", "bat"],
    want: [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]],
  },
  {
    name: "empty string",
    strs: [""],
    want: [[""]],
  },
  {
    name: "single character strings",
    strs: ["a", "b", "a"],
    want: [["a", "a"], ["b"]],
  },
  {
    name: "mixed case strings",
    strs: ["a", "A"],
    want: [["a"], ["A"]],
  },
  {
    name: "no anagrams",
    strs: ["abc", "def", "ghi"],
    want: [["abc"], ["def"], ["ghi"]],
  },
  {
    name: "all anagrams",
    strs: ["abc", "bca", "cab"],
    want: [["abc", "bca", "cab"]],
  },
];

for (const tt of cases) {
  test(tt.name, () => {
    const actual = groupAnagrams(tt.strs);
    actual.forEach((arr) => arr.sort());
    tt.want.forEach((arr) => arr.sort());
    actual.sort((a, b) => a.length - b.length);
    tt.want.sort((a, b) => a.length - b.length);
    expect(actual).toEqual(tt.want);
  });
}
