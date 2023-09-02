import { numUniqueEmails, numUniqueEmails2 } from "./929";

interface tt {
  emails: string[];
  want: number;
}

const tests: tt[] = [
  {
    emails: [
      "test.email+alex@leetcode.com",
      "test.e.mail+bob.cathy@leetcode.com",
      "testemail+david@lee.tcode.com",
    ],
    want: 2,
  },
  {
    emails: ["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"],
    want: 3,
  },
];

describe("unique emails address", () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = numUniqueEmails(t.emails);
      const result2 = numUniqueEmails2(t.emails);
      expect(result).toBe(t.want);
      expect(result2).toBe(t.want);
    });
  });
});
