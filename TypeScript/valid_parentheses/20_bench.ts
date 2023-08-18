import isValid from "./20.ts";

interface tt {
  s: string;
}

const tests: tt[] = [
  { s: "()" },
  { s: "()[]{}" },
  { s: "(]" },
];

for (const t of tests) {
  Deno.bench(`isValid("${t.s}")`, () => {
    isValid(t.s);
  });
}
