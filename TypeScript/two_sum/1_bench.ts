import twoSum from "./1.ts";

interface tt {
  nums: number[];
  target: number;
}

const benchs: tt[] = [
  {
    nums: [2, 7, 11, 15],
    target: 9,
  },
  {
    nums: [3, 2, 4],
    target: 6,
  },
  {
    nums: [2, 3, 4, 1, 25, 8],
    target: 30,
  },
];

for (const t of benchs) {
  Deno.bench(`twoSum([${t.nums}], ${t.target})`, () => {
    twoSum(t.nums, t.target);
  });
}
