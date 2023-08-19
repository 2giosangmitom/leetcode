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

Deno.test("water bottles", () => {
  for (const t of tests) {
    const result = numWaterBottles(t.numBottles, t.numExchange);
    assertEquals(result, t.want);
  }
});
