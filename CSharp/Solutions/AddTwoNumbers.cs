namespace Solutions;

public class ListNode(int val, ListNode? next = null)
{
    public int val = val;
    public ListNode next = next!;
}

public class AddTwoNumbers
{
    public static ListNode AddTwoNumbers1(ListNode? l1, ListNode? l2)
    {
        ListNode dummyHead = new(0);
        ListNode tail = dummyHead;
        int carry = 0;

        while (l1 != null || l2 != null || carry != 0)
        {
            int digit1 = l1?.val ?? 0;
            int digit2 = l2?.val ?? 0;

            int sum = digit1 + digit2 + carry;
            int digit = sum % 10;
            carry = sum / 10;

            ListNode newNode = new(digit, null);
            tail.next = newNode;
            tail = tail.next;

            l1 = l1?.next;
            l2 = l2?.next;
        }

        return dummyHead.next;
    }
}
