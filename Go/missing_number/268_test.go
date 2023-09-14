package missingnumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMissingNumber(t *testing.T) {
	tests := []struct {
		nums []int
		want int
	}{
		{nums: []int{3, 0, 1}, want: 2},
		{nums: []int{0, 1}, want: 2},
		{nums: []int{9, 6, 4, 2, 3, 5, 7, 0, 1}, want: 8},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := missingNumber(tt.nums)
			assert.Equal(t, tt.want, got)
		})
	}
}
