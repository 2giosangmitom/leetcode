package lib

type ListNode struct {
	Next *ListNode
	Val  int
}

func NewList(vals []int) *ListNode {
	head := &ListNode{}
	current := head
	for _, val := range vals {
		current.Next = &ListNode{Val: val}
		current = current.Next
	}
	return head.Next
}

func (l *ListNode) ToSlice() []int {
	slice := []int{}
	for current := l; current != nil; current = current.Next {
		slice = append(slice, current.Val)
	}
	return slice
}
