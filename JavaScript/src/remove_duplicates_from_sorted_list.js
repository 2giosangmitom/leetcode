import { ListNode } from "#lib/linked_list.js";

/**
 * @param {ListNode} head
 * @return {ListNode}
 */
function deleteDuplicates(head) {
  if (!head) {
    return null;
  }
  let dummyHead = head;
  for (let tail = head.next; tail != null; tail = tail.next) {
    if (dummyHead.val != tail.val) {
      dummyHead.next = tail;
      dummyHead = dummyHead.next;
    }
  }
  dummyHead.next = null;

  return head;
}

export { deleteDuplicates };
