import { assertEquals } from "@std/assert";
import { reverse } from "./7.ts";

interface tt {
  x: number;
  want: number;
}

const cases: tt[] = [
  { x: 123, want: 321 },
  { x: -123, want: -321 },
  { x: 120, want: 21 },
  { x: 1534236469, want: 0 },
  { x: -2147483648, want: 0 },
  { x: 900000, want: 9 },
];

for (const t of cases) {
  Deno.test(JSON.stringify(t), () => {
    const result = reverse(t.x);
    assertEquals(result, t.want);
  });
}
