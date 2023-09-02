import plusOne from "./66";

interface tt {
  digits: number[];
  want: number[];
}

let tests: tt[] = [
  { digits: [1, 2, 3], want: [1, 2, 4] },
  { digits: [4, 3, 2, 1], want: [4, 3, 2, 2] },
  { digits: [9], want: [1, 0] },
];

describe("plus one", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = plusOne(t.digits);
      expect(result).toEqual(t.want);
    });
  });
});
