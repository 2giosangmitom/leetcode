import { reverse } from "../src/7.js";
import { assertEquals } from "jsr:@std/assert";

const cases = [
	{ x: 123, want: 321 },
	{ x: -123, want: -321 },
	{ x: 120, want: 21 },
	{ x: 1463847412, want: 2147483641 },
	{ x: -2147483648, want: 0 },
];

Deno.test("7. Reverse Integer", () => {
	for (let i = 0; i < cases.length; i++) {
		const result = reverse(cases[i].x);
		assertEquals(result, cases[i].want);
	}
});
