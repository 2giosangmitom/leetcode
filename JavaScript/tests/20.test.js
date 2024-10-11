import { assertEquals } from "jsr:@std/assert";
import { isValid } from "../src/20.js";

const cases = [
	{ s: "()", want: true },
	{ s: "()[]{}", want: true },
	{ s: "(]", want: false },
	{ s: "([])", want: true },
	{ s: "([}}])", want: false },
];

Deno.test("20. Valid Parentheses", () => {
	for (let i = 0; i < cases.length; i++) {
		const result = isValid(cases[i].s);
		assertEquals(result, cases[i].want);
	}
});
