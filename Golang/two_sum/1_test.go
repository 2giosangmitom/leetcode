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
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := twoSum(tt.nums, tt.target)
			assert.Equal(t, tt.want, result)
		})
	}
}
