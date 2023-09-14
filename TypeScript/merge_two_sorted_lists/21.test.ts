import { ListNode, mergeTwoLists } from './21';
import { describe, test, expect } from 'bun:test';

interface tt {
  list1: ListNode | null;
  list2: ListNode | null;
  want: ListNode | null;
}
const tests: tt[] = [
  {
    list1: new ListNode(1, new ListNode(2, new ListNode(4))),
    list2: new ListNode(1, new ListNode(3, new ListNode(4, null))),
    want: new ListNode(1, new ListNode(1, new ListNode(2, new ListNode(3, new ListNode(4, new ListNode(4, null)))))),
  },
];

describe('merge two sorted lists', () => {
  tests.forEach((t, i) => {
    test(`case ${i + 1}`, () => {
      const result = mergeTwoLists(t.list1, t.list2);
      expect(result).toEqual(t.want);
    });
  });
});
