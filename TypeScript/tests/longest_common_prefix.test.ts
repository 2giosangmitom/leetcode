import { longestCommonPrefix } from "../src/longest_common_prefix.ts";
import { describe, test, expect } from "@jest/globals";

describe("longest common prefix", () => {
  const cases = [
    { strs: ["flower", "flow", "flight"], want: "fl" },
    { strs: ["dog", "racecar", "car"], want: "" },
    { strs: ["chi", "chien", "chau"], want: "ch" },
  ];

  for (const tt of cases) {
    test(JSON.stringify(tt), () => {
      expect(longestCommonPrefix(tt.strs)).toBe(tt.want);
    });
  }
});
