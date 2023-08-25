package addtwonumbers

type ListNode struct {
	Val  int
	Next *ListNode
}

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	dummyHead := ListNode{0, nil}
	tail := &dummyHead
	carry := 0

	getVal := func(l *ListNode) int {
		if l != nil {
			return l.Val
		}
		return 0
	}

	getNext := func(l *ListNode) *ListNode {
		if l != nil {
			return l.Next
		}
		return nil
	}

	for l1 != nil || l2 != nil || carry != 0 {
		digit1 := getVal(l1)
		digit2 := getVal(l2)
		l1 = getNext(l1)
		l2 = getNext(l2)

		sum := digit1 + digit2 + carry
		digit := sum % 10
		carry = sum / 10

		newNode := &ListNode{digit, nil}
		tail.Next = newNode
		tail = tail.Next

	}

	return dummyHead.Next
}
