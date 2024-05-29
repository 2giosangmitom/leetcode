package mergetwosortedlists

import (
	"fmt"
	"testing"

	helpers "leetcode"

	"github.com/stretchr/testify/assert"
)

func TestMergeTwoLists(t *testing.T) {
	cases := []struct {
		list1 ListNode
		list2 ListNode
		want  ListNode
	}{
		{
			list1: helpers.CreateList([]int{1, 2, 4}),
			list2: helpers.CreateList([]int{1, 3, 4}),
			want:  helpers.CreateList([]int{1, 1, 2, 3, 4, 4}),
		},
		{
			list1: helpers.CreateList([]int{2, 3, 5}),
			list2: helpers.CreateList([]int{2, 3, 9}),
			want:  helpers.CreateList([]int{2, 2, 3, 3, 5, 9}),
		},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := mergeTwoLists(&tt.list1, &tt.list2)
			assert.Equal(t, tt.want, *result)
		})
	}
}
