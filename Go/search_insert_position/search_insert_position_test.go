package search_insert_position

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSearchInsertPosition(t *testing.T) {
	cases := []struct {
		name   string
		nums   []int
		target int
		want   int
	}{
		{name: "target present in middle of array", nums: []int{1, 3, 5, 6}, target: 5, want: 2},
		{name: "target greater than all elements", nums: []int{1, 3, 5, 6}, target: 7, want: 4},
		{name: "target less than all elements", nums: []int{1, 3, 5, 6}, target: 0, want: 0},
		{name: "target should be inserted at middle", nums: []int{1, 3, 5, 6}, target: 4, want: 2},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := searchInsert(tt.nums, tt.target)
			assert.Equal(t, tt.want, actual)
		})
	}
}
