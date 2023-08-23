import { ListNode, mergeTwoLists } from "./21.ts";
import { assertEquals } from "../deps.ts";

interface tt {
  list1: ListNode | null;
  list2: ListNode | null;
  want: ListNode | null;
}
const tests: tt[] = [
  {
    list1: new ListNode(1, new ListNode(2, new ListNode(4))),
    list2: new ListNode(1, new ListNode(3, new ListNode(4, null))),
    want: new ListNode(
      1,
      new ListNode(
        1,
        new ListNode(
          2,
          new ListNode(3, new ListNode(4, new ListNode(4, null))),
        ),
      ),
    ),
  },
];

for (const t of tests) {
  Deno.test("merge 2 sorted lists", () => {
    const result = mergeTwoLists(t.list1, t.list2);
    assertEquals(result, t.want);
  });
}
