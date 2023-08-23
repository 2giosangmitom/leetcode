import { assertEquals } from "../deps.ts";
import longestCommonPrefix from "./14.ts";

interface tt {
  strs: string[];
  want: string;
}

const tests: tt[] = [
  {
    strs: ["flower", "flow", "flight"],
    want: "fl",
  },
  {
    strs: ["dog", "racecar", "car"],
    want: "",
  },
];

for (const t of tests) {
  Deno.test("longest common prefix", () => {
    const result = longestCommonPrefix(t.strs);
    assertEquals(result, t.want);
  });
}
