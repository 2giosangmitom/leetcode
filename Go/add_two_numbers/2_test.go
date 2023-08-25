package addtwonumbers

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_addTwoNumbers(t *testing.T) {
	tests := []struct {
		l1   *ListNode
		l2   *ListNode
		want *ListNode
	}{
		{
			l1:   &ListNode{2, &ListNode{4, &ListNode{3, nil}}},
			l2:   &ListNode{5, &ListNode{6, &ListNode{4, nil}}},
			want: &ListNode{7, &ListNode{0, &ListNode{8, nil}}},
		},
		{
			l1:   &ListNode{0, nil},
			l2:   &ListNode{0, nil},
			want: &ListNode{0, nil},
		},
		{
			l1:   &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, nil}}}}}}},
			l2:   &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{9, nil}}}},
			want: &ListNode{8, &ListNode{9, &ListNode{9, &ListNode{9, &ListNode{0, &ListNode{0, &ListNode{0, &ListNode{1, nil}}}}}}}},
		},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := addTwoNumbers(tt.l1, tt.l2)
			assert.Equal(t, tt.want, got)
		})
	}
}
