import { describe, test, expect } from "bun:test";
import { addTwoNumbers, ListNode } from "@/add_two_numbers";

describe("add two numbers", () => {
  const cases = [
    {
      l1: new ListNode(2, new ListNode(4, new ListNode(3, null))),
      l2: new ListNode(5, new ListNode(6, new ListNode(4, null))),
      want: new ListNode(7, new ListNode(0, new ListNode(8, null))),
    },
    {
      l1: new ListNode(0, null),
      l2: new ListNode(0, null),
      want: new ListNode(0, null),
    },
    {
      l1: new ListNode(
        9,
        new ListNode(
          9,
          new ListNode(
            9,
            new ListNode(
              9,
              new ListNode(9, new ListNode(9, new ListNode(9, null))),
            ),
          ),
        ),
      ),
      l2: new ListNode(
        9,
        new ListNode(9, new ListNode(9, new ListNode(9, null))),
      ),
      want: new ListNode(
        8,
        new ListNode(
          9,
          new ListNode(
            9,
            new ListNode(
              9,
              new ListNode(
                0,
                new ListNode(0, new ListNode(0, new ListNode(1, null))),
              ),
            ),
          ),
        ),
      ),
    },
  ];

  for (const [i, tt] of cases.entries()) {
    test(`case ${i}`, () => {
      expect(addTwoNumbers(tt.l1, tt.l2)).toEqual(tt.want);
    });
  }
});
