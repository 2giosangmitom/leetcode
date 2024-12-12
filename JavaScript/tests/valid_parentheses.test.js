import { isValid } from "#src/valid_parentheses.js";
import { expect, test } from "vitest";

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
  test(tt.name, () => {
    const actual = isValid(tt.s);
    expect(actual).toBe(tt.want);
  });
}
