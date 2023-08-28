package removeduplicatesfromsortedarr

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_removeDuplicates(t *testing.T) {
	tests := []struct {
		nums     []int
		want     int
		wantNums []int
	}{
		{nums: []int{1, 1, 2}, want: 2, wantNums: []int{1, 2}},
		{nums: []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}, want: 5, wantNums: []int{0, 1, 2, 3, 4}},
	}
	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := removeDuplicates(tt.nums)
			assert.Equal(t, tt.want, got)
			assert.Equal(t, tt.wantNums, tt.nums[:tt.want])
		})
	}
}
