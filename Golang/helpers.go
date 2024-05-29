package helpers

type ListNode struct {
	Val  int
	Next *ListNode
}

func CreateList(vals []int) ListNode {
	head := &ListNode{0, nil}
	tail := head
	for _, val := range vals {
		tail.Next = &ListNode{val, nil}
		tail = tail.Next
	}
	return *head.Next
}
