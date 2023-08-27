import { assertEquals } from "../deps.ts";
import isValid from "./20.ts";

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

for (const t of tests) {
  Deno.test("valid parentheses", () => {
    const result = isValid(t.s);
    assertEquals(result, t.want);
  });
}
