package remove_element

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_removeElement(t *testing.T) {
	tests := []struct {
		nums []int
		val  int
		want int
	}{
		{nums: []int{3, 2, 2, 3}, val: 3, want: 2},
		{nums: []int{0, 1, 2, 2, 3, 0, 4, 2}, val: 2, want: 5},
	}
	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := removeElement(tt.nums, tt.val)
			assert.Equal(t, tt.want, got)
		})
	}
}
