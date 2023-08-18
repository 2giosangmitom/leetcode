import longestCommonPrefix from "./14.ts";

interface tt {
  strs: string[];
}

const cases: tt[] = [
  {
    strs: ["flower", "flow", "flight"],
  },
  {
    strs: ["dog", "racecar", "car"],
  },
];

for (const t of cases) {
  Deno.bench(`longestCommonPrefix([${t.strs}])`, () => {
    longestCommonPrefix(t.strs);
  });
}
