package removeduplicatesfromsortedarr

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_removeDuplicates(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{nums: []int{1, 1, 2}, want: 2},
		{nums: []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, want: 5},
	}
	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := removeDuplicates(tt.nums)
			assert.Equal(t, tt.want, got)
		})
	}
}
