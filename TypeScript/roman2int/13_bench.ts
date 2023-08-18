import romanToInt from "./13.ts";

interface tt {
  s: string;
}

const benchs: tt[] = [
  { s: "III" },
  { s: "LVIII" },
  { s: "MCMXCIV" },
];

for (const t of benchs) {
  Deno.bench(`romanToInt("${t.s}")`, () => {
    romanToInt(t.s);
  });
}
