package mergetwosortedlists

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMergeTwoLists(t *testing.T) {
	cases := []struct {
		list1 ListNode
		list2 ListNode
		want  ListNode
	}{
		{
			list1: ListNode{1, &ListNode{2, &ListNode{4, nil}}},
			list2: ListNode{1, &ListNode{3, &ListNode{4, nil}}},
			want:  ListNode{1, &ListNode{1, &ListNode{2, &ListNode{3, &ListNode{4, &ListNode{4, nil}}}}}},
		},
		{
			list1: ListNode{2, &ListNode{3, &ListNode{5, nil}}},
			list2: ListNode{2, &ListNode{3, &ListNode{9, nil}}},
			want:  ListNode{2, &ListNode{2, &ListNode{3, &ListNode{3, &ListNode{5, &ListNode{9, nil}}}}}},
		},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := mergeTwoLists(&tt.list1, &tt.list2)
			assert.Equal(t, tt.want, *result)
		})
	}
}
