import { ListNode } from "#lib/linked_list";
import { addTwoNumbers } from "#src/add_two_numbers";
import { expect, test } from "vitest";

const cases = [
  {
    name: "same number of digits",
    l1: ListNode.fromArray([2, 4, 3]),
    l2: ListNode.fromArray([5, 6, 4]),
    want: [7, 0, 8],
  },
  {
    name: "different number of digits",
    l1: ListNode.fromArray([2, 4, 3]),
    l2: ListNode.fromArray([5, 6]),
    want: [7, 0, 4],
  },
  {
    name: "one list is empty",
    l1: ListNode.fromArray([]),
    l2: ListNode.fromArray([5, 6]),
    want: [5, 6],
  },
  {
    name: "both lists are empty",
    l1: ListNode.fromArray([]),
    l2: ListNode.fromArray([]),
    want: [],
  },
];

for (const c of cases) {
  test(c.name, () => {
    const result = addTwoNumbers(c.l1, c.l2);
    expect(result?.toArray() ?? []).toEqual(c.want);
  });
}
