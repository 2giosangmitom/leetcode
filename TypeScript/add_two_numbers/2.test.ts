import { assertEquals } from "@std/assert";
import { addTwoNumbers, ListNode } from "./2.ts";

interface tt {
  l1: ListNode;
  l2: ListNode;
  want: ListNode;
}

const cases: tt[] = [
  {
    l1: new ListNode(2, new ListNode(4, new ListNode(3))),
    l2: new ListNode(5, new ListNode(6, new ListNode(4))),
    want: new ListNode(7, new ListNode(0, new ListNode(8))),
  },
  {
    l1: new ListNode(
      9,
      new ListNode(
        9,
        new ListNode(
          9,
          new ListNode(9, new ListNode(9, new ListNode(9, new ListNode(9)))),
        ),
      ),
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
            new ListNode(0, new ListNode(0, new ListNode(0, new ListNode(1)))),
          ),
        ),
      ),
    ),
  },
];

for (const t of cases) {
  Deno.test(JSON.stringify(t), () => {
    const result = addTwoNumbers(t.l1, t.l2);
    assertEquals(result, t.want);
  });
}
