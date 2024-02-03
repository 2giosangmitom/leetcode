package singlenumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_136(t *testing.T) {
	cases := []struct {
		nums []int
		want int
	}{
		{nums: []int{2, 2, 1}, want: 1},
		{nums: []int{4, 1, 2, 1, 2}, want: 4},
		{nums: []int{1}, want: 1},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := singleNumber(tt.nums)
			assert.Equal(t, tt.want, result)
		})
	}
}
