package two_sum

import (
	"fmt"
	"leetcode/pkg/assert"
	"testing"
)

func Test_1_Two_Sum(t *testing.T) {
	cases := []struct {
		nums   []int
		target int
		want   []int
	}{
		{
			nums:   []int{2, 7, 11, 15},
			target: 9,
			want:   []int{0, 1},
		},
		{
			nums:   []int{3, 2, 4},
			target: 6,
			want:   []int{1, 2},
		},
		{
			nums:   []int{3, 3},
			target: 6,
			want:   []int{0, 1},
		},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			actual := twoSum(tt.nums, tt.target)
			assert.Equal(t, actual, tt.want)
		})
	}
}
