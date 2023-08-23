import { assertEquals } from "../deps.ts";
import numWaterBottles from "./1518.ts";

interface tt {
  numBottles: number;
  numExchange: number;
  want: number;
}

const tests: tt[] = [
  { numBottles: 9, numExchange: 3, want: 13 },
  { numBottles: 15, numExchange: 4, want: 19 },
];

for (const t of tests) {
  Deno.test("water bottles", () => {
    const result = numWaterBottles(t.numBottles, t.numExchange);
    assertEquals(result, t.want);
  });
}
