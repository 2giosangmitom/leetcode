package addtwonumbers

import (
	"fmt"
	"testing"

	helpers "leetcode"

	"github.com/stretchr/testify/assert"
)

func TestAddTwoNumbers(t *testing.T) {
	cases := []struct {
		l1   ListNode
		l2   ListNode
		want ListNode
	}{
		{
			l1:   helpers.CreateList([]int{2, 4, 3}),
			l2:   helpers.CreateList([]int{5, 6, 4}),
			want: helpers.CreateList([]int{7, 0, 8}),
		},
		{
			l1:   helpers.CreateList([]int{0}),
			l2:   helpers.CreateList([]int{0}),
			want: helpers.CreateList([]int{0}),
		},
		{
			l1:   helpers.CreateList([]int{9, 9, 9, 9, 9, 9, 9}),
			l2:   helpers.CreateList([]int{9, 9, 9, 9}),
			want: helpers.CreateList([]int{8, 9, 9, 9, 0, 0, 0, 1}),
		},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := addTwoNumbers(&tt.l1, &tt.l2)
			assert.Equal(t, &tt.want, result)
		})
	}
}
