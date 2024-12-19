/**
 * @param {import("../lib/linked_list").ListNode} head
 * @returns {import("../lib/linked_list").ListNode}
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
