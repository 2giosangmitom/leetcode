package add_two_numbers

import (
	"leetcode/lib"
)

type ListNode = lib.ListNode

func addTwoNumbers(l1 *ListNode, l2 *ListNode) *ListNode {
	head := &ListNode{Val: 0, Next: nil}
	tail := head
	carry := 0

	for l1 != nil || l2 != nil || carry != 0 {
		digit1, digit2 := 0, 0

		if l1 != nil {
			digit1 = l1.Val
			l1 = l1.Next
		}
		if l2 != nil {
			digit2 = l2.Val
			l2 = l2.Next
		}

		sum := digit1 + digit2 + carry
		lastDigit := sum % 10
		carry = sum / 10

		newNode := &ListNode{Val: lastDigit, Next: nil}
		tail.Next = newNode
		tail = newNode
	}

	return head.Next
}
