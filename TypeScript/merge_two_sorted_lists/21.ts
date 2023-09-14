class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function mergeTwoLists(list1: ListNode | null, list2: ListNode | null): ListNode | null {
  const head = new ListNode();
  let curr = head,
    p1 = list1,
    p2 = list2;

  while (p1 !== null && p2 !== null) {
    if (p1.val < p2.val) {
      curr.next = p1;
      p1 = p1.next;
    } else {
      curr.next = p2;
      p2 = p2.next;
    }
    curr = curr.next;
  }
  curr.next = p1 ?? p2;

  return head.next;
}

export { ListNode, mergeTwoLists };
