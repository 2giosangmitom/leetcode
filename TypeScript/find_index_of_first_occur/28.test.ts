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
];

Deno.test("strStr", () => {
  for (const t of tests) {
    const result = strStr(t.haystack, t.needle);
    assertEquals(result, t.want);
  }
});
