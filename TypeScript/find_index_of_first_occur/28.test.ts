import { assertEquals } from "../deps.ts";
import strStr from "./28.ts";

interface tt {
  haystack: string;
  needle: string;
  want: number;
}

const tests: tt[] = [
  { haystack: "sadbutsad", needle: "sad", want: 0 },
  { haystack: "leetcode", needle: "leeto", want: -1 },
  { haystack: "hello", needle: "ll", want: 2 },
  { haystack: "a", needle: "a", want: 0 },
  { haystack: "abc", needle: "c", want: 2 },
  { haystack: "baaa", needle: "aaaa", want: -1 },
];

for (const t of tests) {
  Deno.test("strStr", () => {
    const result = strStr(t.haystack, t.needle);
    assertEquals(result, t.want);
  });
}
