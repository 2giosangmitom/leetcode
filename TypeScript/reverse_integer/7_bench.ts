import reverse from "./7.ts";

interface tt {
  num: number;
}

const benchs: tt[] = [
  { num: 123 },
  { num: 900000 },
  { num: -123 },
  { num: 553126124224 },
];

for (const t of benchs) {
  Deno.bench(`7. reverse(${t.num})`, () => {
    reverse(t.num);
  });
}
