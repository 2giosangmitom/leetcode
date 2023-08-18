import romanToInt from "./13.ts";

interface tt {
  s: string;
}

const cases: tt[] = [
  { s: "III" },
  { s: "LVIII" },
  { s: "MCMXCIV" },
];

for (const t of cases) {
  Deno.bench(`romanToInt("${t.s}")`, () => {
    romanToInt(t.s);
  });
}
