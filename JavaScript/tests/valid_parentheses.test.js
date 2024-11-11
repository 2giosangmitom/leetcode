import { isValid } from "../src/valid_parentheses.js";
import { assertEquals } from "@std/assert";

const cases = [
  {
    name: "valid simple parentheses",
    s: "()",
    want: true,
  },
  {
    name: "valid multiple types",
    s: "()[]{}",
    want: true,
  },
  {
    name: "invalid mixed parentheses",
    s: "(]",
    want: false,
  },
  {
    name: "valid nested parentheses",
    s: "([])",
    want: true,
  },
  {
    name: "invalid nested parentheses",
    s: "([}}])",
    want: false,
  },
  {
    name: "invalid single opening bracket",
    s: "(",
    want: false,
  },
  {
    name: "invalid single closing bracket",
    s: ")",
    want: false,
  },
];

for (const tt of cases) {
  Deno.test(tt.name, () => {
    const actual = isValid(tt.s);
    assertEquals(actual, tt.want);
  });
}
