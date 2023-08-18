import { assertEquals } from "../deps.ts";
import isValid from "./20.ts";

interface tt {
  s: string;
  want: boolean;
}

Deno.test("valid parentheses", () => {
  const tests: tt[] = [
    { s: "()", want: true },
    { s: "()[]{}", want: true },
    { s: "(]", want: false },
  ];

  for (const t of tests) {
    const result = isValid(t.s);
    assertEquals(result, t.want);
  }
});
