namespace add_two_numbers;

public class ListNode {
  public int val;
  public ListNode next;
  public ListNode(int val = 0, ListNode next = null) {
    this.val = val;
    this.next = next;
  }
}

public class Solution {
  public static ListNode AddTwoNumbers(ListNode l1, ListNode l2) {
    var dummyHead = new ListNode(0, null);
    var tail = dummyHead;
    int carry = 0;

    while (l1 != null || l2 != null || carry != 0) {
      int digit1 = l1 != null ? l1.val : 0;
      int digit2 = l2 != null ? l2.val : 0;
      l1 = l1?.next;
      l2 = l2?.next;

      int sum = digit1 + digit2 + carry;
      int digit = sum % 10;
      carry = sum / 10;

      var newNode = new ListNode(digit, null);
      tail.next = newNode;
      tail = tail.next;
    }

    return dummyHead.next;
  }
}

