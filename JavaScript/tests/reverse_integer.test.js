import { reverse } from "../src/reverse_integer.js";
import { assertEquals } from "@std/assert";

const cases = [
  {
    name: "positive integer",
    x: 123,
    want: 321,
  },
  {
    name: "negative integer",
    x: -123,
    want: -321,
  },
  {
    name: "integer with trailing zeros",
    x: 120,
    want: 21,
  },
  {
    name: "encounter overflow",
    x: -2147483648,
    want: 0,
  },
  {
    name: "encounter overflow",
    x: 2147483647,
    want: 0,
  },
];

for (const tt of cases) {
  Deno.test(tt.name, () => {
    const actual = reverse(tt.x);
    assertEquals(actual, tt.want);
  });
}
