import numWaterBottles from "./1518.ts";

interface tt {
  numBottles: number;
  numExchange: number;
}

const tests: tt[] = [
  { numBottles: 9, numExchange: 3 },
  { numBottles: 15, numExchange: 4 },
];

for (const t of tests) {
  Deno.bench(`numWaterBottles(${t.numBottles}, ${t.numExchange})`, () => {
    numWaterBottles(t.numBottles, t.numExchange);
  });
}
