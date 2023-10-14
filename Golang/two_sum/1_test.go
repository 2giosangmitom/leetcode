package twosum

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_1(t *testing.T) {
	cases := []struct {
		nums   []int
		target int
		want   []int
	}{
		{nums: []int{2, 7, 11, 15}, target: 9, want: []int{0, 1}},
		{nums: []int{3, 2, 4}, target: 6, want: []int{1, 2}},
		{nums: []int{3, 3}, target: 6, want: []int{0, 1}},
		{nums: []int{2, 3, 4, 1, 25, 8}, target: 30, want: []int{-1}},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := twoSum(tt.nums, tt.target)
			result2 := twoSum2(tt.nums, tt.target)
			assert.Equal(t, tt.want, result)
			assert.Equal(t, tt.want, result2)
		})
	}
}
