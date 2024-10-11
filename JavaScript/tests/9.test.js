import { assertEquals } from "jsr:@std/assert";
import { isPalindrome } from "../src/9.js";

const cases = [
	{ x: 121, want: true },
	{ x: -121, want: false },
	{ x: 10, want: false },
];

Deno.test("9. Palindrome Number", () => {
	for (let i = 0; i < cases.length; i++) {
		assertEquals(isPalindrome(cases[i].x), cases[i].want);
	}
});
