package merge2sortedlists

import (
	"reflect"
	"testing"
)

func TestMerge2SortedLists(t *testing.T) {
	t.Run("case 0", func(t *testing.T) {
		list1 := &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 4}}}
		list2 := &ListNode{Val: 1, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4}}}
		got := mergeTwoLists(list1, list2)
		want := &ListNode{Val: 1, Next: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: &ListNode{Val: 4}}}}}}

		if !reflect.DeepEqual(got, want) {
			t.Errorf("Got %v but want %v", got, want)
		}
	})
}
