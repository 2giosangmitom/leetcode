package removeduplicatesfromsortedarray

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRemoveDuplicates(t *testing.T) {
	cases := []struct {
		nums    []int
		want    int
		wantArr []int
	}{
		{nums: []int{1, 1, 2}, want: 2, wantArr: []int{1, 2}},
		{nums: []int{0, 0, 1, 1, 1, 1, 2, 3, 4}, want: 5, wantArr: []int{0, 1, 2, 3, 4}},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := removeDuplicates(tt.nums)
			assert.Equal(t, tt.want, result)
			assert.Equal(t, tt.wantArr, tt.nums[:len(tt.wantArr)])
		})
	}
}
