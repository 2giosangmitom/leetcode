package add_two_numbers

import (
	"leetcode/lib"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAddTwoNumbers(t *testing.T) {
	cases := []struct {
		name     string
		l1       *ListNode
		l2       *ListNode
		expected []int
	}{
		{name: "same number of digits", l1: lib.NewList([]int{2, 4, 3}), l2: lib.NewList([]int{5, 6, 4}), expected: []int{7, 0, 8}},
		{name: "different number of digits", l1: lib.NewList([]int{2, 4, 3}), l2: lib.NewList([]int{5, 6}), expected: []int{7, 0, 4}},
		{name: "one list is empty", l1: lib.NewList([]int{}), l2: lib.NewList([]int{5, 6}), expected: []int{5, 6}},
		{name: "both lists are empty", l1: lib.NewList([]int{}), l2: lib.NewList([]int{}), expected: []int{}},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			actual := addTwoNumbers(c.l1, c.l2)
			assert.Equal(t, c.expected, actual.ToSlice())
		})
	}
}
