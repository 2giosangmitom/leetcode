package lib

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLinkedList(t *testing.T) {
	cases := []struct {
		want *ListNode
		vals []int
	}{
		{
			vals: []int{1, 2, 3, 4},
			want: &ListNode{Val: 1, Next: &ListNode{Val: 2, Next: &ListNode{Val: 3, Next: &ListNode{Val: 4, Next: nil}}}},
		},
		{
			vals: []int{},
      want: nil,
		},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			actual := NewList(tt.vals)
			assert.Equal(t, tt.want, actual)
      assert.Equal(t, tt.vals, tt.want.ToSlice())
		})
	}
}
