// Definition for singly-linked list.
class ListNode {
  int val;
  ListNode? next;
  ListNode([this.val = 0, this.next]);
}

class Solution {
  ListNode? addTwoNumbers(ListNode? l1, ListNode? l2) {
    ListNode? dummyHead = ListNode(0);
    ListNode? tail = dummyHead;
    int carry = 0;

    while (l1 != null || l2 != null || carry != 0) {
      int digit1 = l1?.val ?? 0;
      int digit2 = l2?.val ?? 0;

      int sum = digit1 + digit2 + carry;
      int digit = sum % 10;
      carry = (sum / 10).floor();

      var newNode = ListNode(digit, null);
      tail?.next = newNode;
      tail = tail?.next;

      l1 = l1?.next ?? null;
      l2 = l2?.next ?? null;
    }

    return dummyHead.next;
  }
}
