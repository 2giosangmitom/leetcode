package numofgoodpairs

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumOfGoodPairs(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{nums: []int{1, 2, 3, 1, 1, 3}, want: 4},
		{nums: []int{1, 1, 1, 1}, want: 6},
		{nums: []int{1, 2, 3}, want: 0},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := numIdenticalPairs(tt.nums)
			assert.Equal(t, tt.want, got)
		})
	}
}
