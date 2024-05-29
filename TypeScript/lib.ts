export class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

export function createList(vals: number[]) {
  const head = new ListNode();
  let tail = head;
  for (let i = 0; i < vals.length; i++) {
    tail.next = new ListNode(vals[i]);
    tail = tail.next;
  }
  return head.next;
}
