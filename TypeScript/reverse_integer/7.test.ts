import { assertEquals } from "../deps.ts";
import reverse from "./7.ts";

interface tt {
  num: number;
  want: number;
}

const tests: tt[] = [
  { num: 123, want: 321 },
  { num: 900000, want: 9 },
  { num: -123, want: -321 },
  { num: 553126124224, want: 0 },
];

for (const t of tests) {
  Deno.test("reverse integer", () => {
    const result = reverse(t.num);
    assertEquals(result, t.want);
  });
}
