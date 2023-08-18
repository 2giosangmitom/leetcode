import { assertEquals } from "../deps.ts";
import romanToInt from "./13.ts";

interface tt {
  s: string;
  want: number;
}

Deno.test("roman to integer", () => {
  const tests: tt[] = [
    { s: "III", want: 3 },
    { s: "LVIII", want: 58 },
    { s: "MCMXCIV", want: 1994 },
  ];

  for (const t of tests) {
    const result = romanToInt(t.s);
    assertEquals(result, t.want);
  }
});
