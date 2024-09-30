package two_sum

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestTwoSum(t *testing.T) {
	cases := []struct {
		name   string
		nums   []int
		target int
		want   []int
	}{
		{
			name:   "CaseWithValidIndices",
			nums:   []int{2, 7, 11, 15},
			target: 9,
			want:   []int{0, 1},
		},
		{
			name:   "CaseWithDifferentOrder",
			nums:   []int{3, 2, 4},
			target: 6,
			want:   []int{1, 2},
		},
		{
			name:   "CaseWithSameNumbers",
			nums:   []int{3, 3},
			target: 6,
			want:   []int{0, 1},
		},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := twoSum(tt.nums, tt.target)
			assert.DeepEqual(t, actual, tt.want)
		})
	}
}
