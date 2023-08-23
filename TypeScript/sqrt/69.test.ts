import { assertEquals } from "../deps.ts";
import mySqrt from "./69.ts";

interface tt {
  x: number;
  want: number;
}

const cases: tt[] = [
  { x: 4, want: 2 },
  { x: 8, want: 2 },
];

for (const t of cases) {
  Deno.test("sqrt", () => {
    const result = mySqrt(t.x);
    assertEquals(result, t.want);
  });
}
