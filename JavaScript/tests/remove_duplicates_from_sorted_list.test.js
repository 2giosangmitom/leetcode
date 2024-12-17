import { deleteDuplicates } from "#src/remove_duplicates_from_sorted_list.js";
import { ListNode } from "#lib/linked_list.js";
import { expect, test } from "vitest";

const cases = [
  { head: ListNode.fromArray([1, 1, 2]), want: [1, 2] },
  { head: ListNode.fromArray([1, 1, 2, 3, 3, 3, 3, 3, 3]), want: [1, 2, 3] },
  { head: ListNode.fromArray([]), want: [] },
  {
    head: ListNode.fromArray([-2, -1, -1, 0, 0, 1, 2]),
    want: [-2, -1, 0, 1, 2],
  },
];

cases.forEach(({ head, want }, i) => {
  test(`case ${i + 1}`, () => {
    const actual = deleteDuplicates(head);
    expect(actual?.toArray() ?? []).toEqual(want);
  });
});
