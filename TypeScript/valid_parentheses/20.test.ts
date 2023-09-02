import isValid from "./20";

interface tt {
  s: string;
  want: boolean;
}

const tests: tt[] = [
  { s: "()", want: true },
  { s: "()[]{}", want: true },
  { s: "(]", want: false },
  { s: "}}()", want: false },
];

describe("valid parentheses", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = isValid(t.s);
      expect(result).toBe(t.want);
    });
  });
});
