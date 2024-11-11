package two_sum

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTwoSum(t *testing.T) {
	cases := []struct {
		name   string
		nums   []int
		target int
		want   []int
	}{
		{
			name:   "different numbers",
			nums:   []int{2, 7, 11, 15},
			target: 9,
			want:   []int{0, 1},
		},
		{
			name:   "different order",
			nums:   []int{3, 2, 4},
			target: 6,
			want:   []int{1, 2},
		},
		{
			name:   "same numbers",
			nums:   []int{3, 3},
			target: 6,
			want:   []int{0, 1},
		},
		{
			name:   "no solution",
			nums:   []int{1, 2, 3},
			target: 10,
			want:   []int{},
		},
		{
			name:   "empty slice",
			nums:   []int{},
			target: 5,
			want:   []int{},
		},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := twoSum(tt.nums, tt.target)
			assert.Equal(t, tt.want, actual, "Test case %s failed", tt.name)
		})
	}
}
