class ListNode {
  val: number;
  next: ListNode | null;
  constructor(val?: number, next?: ListNode | null) {
    this.val = val === undefined ? 0 : val;
    this.next = next === undefined ? null : next;
  }
}

function addTwoNumbers(l1: ListNode | null, l2: ListNode | null): ListNode | null {
  const dummyHead = new ListNode(0);
  let tail = dummyHead;
  let carry = 0;

  while (l1 !== null || l2 !== null || carry !== 0) {
    const digit1 = l1?.val ?? 0;
    const digit2 = l2?.val ?? 0;

    const sum = digit1 + digit2 + carry;
    const digit = sum % 10;
    carry = Math.floor(sum / 10);

    const newNode = new ListNode(digit, null);
    tail.next = newNode;
    tail = tail.next;

    l1 = l1?.next ?? null;
    l2 = l2?.next ?? null;
  }

  return dummyHead.next;
}

export { addTwoNumbers, ListNode };
