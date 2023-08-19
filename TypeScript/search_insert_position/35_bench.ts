import searchInsert from "./35.ts";

interface tt {
  nums: number[];
  target: number;
}

const tests: tt[] = [
  { nums: [1, 3, 5, 6], target: 5 },
  { nums: [1, 3, 5, 6], target: 2 },
  { nums: [1, 3, 5, 6], target: 7 },
];

for (const t of tests) {
  Deno.bench(`searchInsert([${t.nums}], ${t.target})`, () => {
    searchInsert(t.nums, t.target);
  });
}
