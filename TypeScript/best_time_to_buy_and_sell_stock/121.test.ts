import { assertEquals } from "../deps.ts";
import maxProfit from "./121.ts";

interface tt {
  prices: number[];
  want: number;
}

const cases: tt[] = [
  { prices: [7, 1, 5, 3, 6, 4], want: 5 },
  { prices: [7, 6, 4, 3, 1], want: 0 },
];

Deno.test("best time to buy and sell stock", () => {
  for (const t of cases) {
    const result = maxProfit(t.prices);
    assertEquals(result, t.want);
  }
});
