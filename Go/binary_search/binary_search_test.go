package binary_search

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBinarySearch(t *testing.T) {
	cases := []struct {
		name   string
		nums   []int
		target int
		want   int
	}{
		{name: "positive numbers", nums: []int{0, 3, 5, 9, 12}, target: 9, want: 3},
		{name: "negative numbers", nums: []int{-12, -9, -5, -3, 0}, target: -9, want: 1},
		{name: "mixed numbers", nums: []int{-12, -9, 0, 3, 5, 9, 12}, target: 0, want: 2},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := search(tt.nums, tt.target)
			assert.Equal(t, tt.want, actual)
		})
	}
}
