package leetcode.AddTwoNumbers;

import leetcode.Helpers.ListNode;

class Solution {
  public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
    ListNode head = new ListNode();
    ListNode tail = head;
    int carry = 0;

    while (l1 != null || l2 != null || carry != 0) {
      int digit1 = l1 != null ? l1.val : 0;
      int digit2 = l2 != null ? l2.val : 0;
      l1 = l1 != null ? l1.next : null;
      l2 = l2 != null ? l2.next : null;

      int sum = digit1 + digit2 + carry;
      int lastDigit = sum % 10;
      carry = sum / 10;

      ListNode newNode = new ListNode(lastDigit);
      tail.next = newNode;
      tail = newNode;
    }

    return head.next;
  }
}
