import { ListNode } from "../lib/linked_list.js";

/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
function addTwoNumbers(l1, l2) {
  const head = new ListNode();
  let tail = head;
  let carry = 0;

  while (l1 != null || l2 != null || carry != 0) {
    const digit1 = l1?.val ?? 0;
    const digit2 = l2?.val ?? 0;
    l1 = l1?.next;
    l2 = l2?.next;

    const sum = digit1 + digit2 + carry;
    const lastDigit = sum % 10;
    carry = Math.floor(sum / 10);

    const newNode = new ListNode(lastDigit);
    tail.next = newNode;
    tail = tail.next;
  }

  return head.next;
}

export { addTwoNumbers };
