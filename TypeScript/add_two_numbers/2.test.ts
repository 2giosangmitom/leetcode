import { addTwoNumbers, ListNode } from "./2";

interface tt {
  l1: ListNode | null;
  l2: ListNode | null;
  want: ListNode | null;
}

const tests: tt[] = [
  {
    l1: new ListNode(2, new ListNode(4, new ListNode(3))),
    l2: new ListNode(5, new ListNode(6, new ListNode(4))),
    want: new ListNode(7, new ListNode(0, new ListNode(8))),
  },
  {
    l1: new ListNode(0),
    l2: new ListNode(0),
    want: new ListNode(0),
  },
  {
    l1: new ListNode(
      9,
      new ListNode(
        9,
        new ListNode(
          9,
          new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9))))
        )
      )
    ),
    l2: new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9)))),
    want: new ListNode(
      8,
      new ListNode(
        9,
        new ListNode(
          9,
          new ListNode(
            9,
            new ListNode(0, new ListNode(0, new ListNode(0, new ListNode(1))))
          )
        )
      )
    ),
  },
];

describe("add two numbers", () => {
  tests.forEach((v, i) => {
    test(`case ${i + 1}`, () => {
      const result = addTwoNumbers(v.l1, v.l2);
      expect(result).toEqual(v.want);
    });
  });
});
