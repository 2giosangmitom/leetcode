import isPalindrome from "./9.ts";

interface tt {
  num: number;
}

const benchs: tt[] = [
  { num: 121 },
  { num: -321 },
  { num: 10 },
];

for (const t of benchs) {
  Deno.bench(`isPalindrome(${t.num})`, () => {
    isPalindrome(t.num);
  });
}
