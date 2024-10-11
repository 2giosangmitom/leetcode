import { twoSum } from "../src/1.js";
import { assertEquals } from "jsr:@std/assert";

const cases = [
	{
		nums: [2, 7, 11, 15],
		target: 9,
		want: [0, 1],
	},
	{
		nums: [3, 2, 4],
		target: 6,
		want: [1, 2],
	},
	{
		nums: [3, 3],
		target: 6,
		want: [0, 1],
	},
];

Deno.test("1. Two Sum", () => {
	for (let i = 0; i < cases.length; i++) {
		const { nums, target, want } = cases[i];
		const result = twoSum(nums, target);
		assertEquals(result, want, `case ${i + 1}`);
	}
});
