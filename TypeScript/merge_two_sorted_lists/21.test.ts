import { ListNode, mergeTwoLists } from "./21.ts";
import { assertEquals } from "../deps.ts";

interface tt {
  list1: ListNode | null;
  list2: ListNode | null;
  want: ListNode | null;
}

Deno.test("merge 2 sorted lists", () => {
  const list1 = new ListNode(1, new ListNode(2, new ListNode(4, null)));
  const list2 = new ListNode(1, new ListNode(3, new ListNode(4, null)));
  const want = new ListNode(
    1,
    new ListNode(
      1,
      new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4, null)))),
    ),
  );
  const tests: tt[] = [
    { list1, list2, want },
  ];

  for (const t of tests) {
    const result = mergeTwoLists(t.list1, t.list2);
    assertEquals(result, t.want);
  }
});
