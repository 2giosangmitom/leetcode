import { describe, test } from "node:test";
import { groupAnagrams } from "../src/49.js";

/**
 * @param {string[][]} result
 * @param {string[][]} expected
 */
function assertEqualAnagrams(result, expected) {
	if (result.length !== expected.length) {
		throw new Error(
			`Assertion failed!\n Expect: ${JSON.stringify(expected)}\n Actual: ${JSON.stringify(result)}`,
		);
	}

	for (let i = 0; i < expected.length; i++) {
		let found = false;

		for (let j = 0; j < result.length; j++) {
			if (expected[i].length !== result[j].length) {
				continue;
			}

			found = expected[i].every((str) => result[j].includes(str));
			if (found) {
				break;
			}
		}

		if (!found) {
			throw new Error(
				`Assertion failed!\n Expect: ${JSON.stringify(expected)}\n Actual: ${JSON.stringify(result)}`,
			);
		}
	}
}

const cases = [
	{
		strs: ["eat", "tea", "tan", "ate", "nat", "bat"],
		want: [["bat"], ["nat", "tan"], ["ate", "eat", "tea"]],
	},
	{
		strs: ["a"],
		want: [["a"]],
	},
	{
		strs: [""],
		want: [[""]],
	},
];

describe("49. Group Anagrams", () => {
	for (let i = 0; i < cases.length; i++) {
		test(`case ${i + 1}`, () => {
			const result = groupAnagrams(cases[i].strs);
			assertEqualAnagrams(result, cases[i].want);
		});
	}
});
