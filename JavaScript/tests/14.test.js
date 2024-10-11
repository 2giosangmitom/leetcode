import { longestCommonPrefix } from "../src/14.js";
import { assertEquals } from "jsr:@std/assert";

const cases = [
	{ strs: ["flower", "flow", "flight"], want: "fl" },
	{ strs: ["dog", "racecar", "car"], want: "" },
];

Deno.test("14. Longest Common Prefix", () => {
	for (let i = 0; i < cases.length; i++) {
		const result = longestCommonPrefix(cases[i].strs);
		assertEquals(result, cases[i].want);
	}
});
