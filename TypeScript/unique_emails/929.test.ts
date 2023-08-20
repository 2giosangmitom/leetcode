import { assertEquals } from "../deps.ts";
import numUniqueEmails from "./929.ts";

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
    emails: [
      "a@leetcode.com",
      "b@leetcode.com",
      "c@leetcode.com",
    ],
    want: 3,
  },
];

Deno.test("unique email address", () => {
  for (const t of tests) {
    const result = numUniqueEmails(t.emails);
    assertEquals(result, t.want);
  }
});
