import { assertEquals } from "https://deno.land/std@0.198.0/assert/mod.ts";
import reverse from "./7.ts";

interface tt {
  num: number;
  want: number;
}

Deno.test("reverse integer", () => {
  const tests: tt[] = [
    { num: 123, want: 321 },
    { num: 900000, want: 9 },
    { num: -123, want: -321 },
    { num: 553126124224, want: 0 },
  ];

  for (const t of tests) {
    const result = reverse(t.num);
    assertEquals(result, t.want);
  }
});
