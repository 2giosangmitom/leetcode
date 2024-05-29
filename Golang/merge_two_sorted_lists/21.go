package mergetwosortedlists

import (
	helpers "leetcode"
)

type ListNode = helpers.ListNode

func mergeTwoLists(list1 *ListNode, list2 *ListNode) *ListNode {
	head := ListNode{}
	prev := &head

	for list1 != nil && list2 != nil {
		if list1.Val < list2.Val {
			prev.Next = list1
			list1 = list1.Next
		} else {
			prev.Next = list2
			list2 = list2.Next
		}
		prev = prev.Next
	}

	if list1 != nil {
		prev.Next = list1
	} else {
		prev.Next = list2
	}

	return head.Next
}
