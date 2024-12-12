import { numWaterBottles } from "../src/water_bottles.js";
import { expect, test } from "vitest";

const cases = [
  {
    name: "basic exchange",
    bottles: 9,
    exchange: 3,
    want: 13,
  },
  {
    name: "multiple exchanges",
    bottles: 15,
    exchange: 4,
    want: 19,
  },
  {
    name: "no exchange",
    bottles: 1,
    exchange: 2,
    want: 1,
  },
  {
    name: "high exchange rate",
    bottles: 5,
    exchange: 10,
    want: 5,
  },
  {
    name: "exact exchange once",
    bottles: 4,
    exchange: 4,
    want: 5,
  },
  {
    name: "large multiple exchanges",
    bottles: 20,
    exchange: 3,
    want: 29,
  },
];

for (const tt of cases) {
  test(tt.name, () => {
    const actual = numWaterBottles(tt.bottles, tt.exchange);
    expect(actual).toBe(tt.want);
  });
}
