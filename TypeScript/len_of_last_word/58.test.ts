import lengthOfLastWord from "./58";

interface tt {
  s: string;
  want: number;
}

const tests: tt[] = [
  { s: "Hello World", want: 5 },
  { s: "   fly me   to   the moon  ", want: 4 },
  { s: "luffy is still joyboy", want: 6 },
];

describe("length of last word", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = lengthOfLastWord(t.s);
      expect(result).toBe(t.want);
    });
  });
});
