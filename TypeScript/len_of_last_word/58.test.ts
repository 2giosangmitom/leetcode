import { assertEquals } from "../deps.ts";
import lengthOfLastWord from "./58.ts";

interface tt {
  s: string;
  want: number;
}

const cases: tt[] = [
  { s: "Hello World", want: 5 },
  { s: "   fly me   to   the moon  ", want: 4 },
  { s: "luffy is still joyboy", want: 6 },
];

Deno.test("length of last word", () => {
  for (const t of cases) {
    const result = lengthOfLastWord(t.s);
    assertEquals(result, t.want);
  }
});
