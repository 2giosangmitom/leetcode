package leetcode.Helpers;

public class ListNode {
  public int val;
  public ListNode next;

  public ListNode() {}

  public ListNode(int val) {
    this.val = val;
  }

  public ListNode(int val, ListNode next) {
    this.val = val;
    this.next = next;
  }

  public static ListNode fromArray(int[] arr) {
    if (arr.length == 0) {
      return null;
    }
    ListNode head = new ListNode(arr[0]);
    ListNode current = head;
    for (int i = 1; i < arr.length; i++) {
      current.next = new ListNode(arr[i]);
      current = current.next;
    }
    return head;
  }

  public int[] toArray() {
    int length = 0;
    ListNode current = this;
    while (current != null) {
      length++;
      current = current.next;
    }
    int[] arr = new int[length];
    current = this;
    for (int i = 0; i < length; i++) {
      arr[i] = current.val;
      current = current.next;
    }
    return arr;
  }
}
