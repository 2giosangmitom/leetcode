package single_number

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSingleNumber(t *testing.T) {
	cases := []struct {
		name string
		nums []int
		want int
	}{
		{name: "same numbers twice", nums: []int{2, 2, 1}, want: 1},
		{name: "same numbers interleaved", nums: []int{4, 1, 2, 1, 2}, want: 4},
		{name: "one element", nums: []int{1}, want: 1},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := singleNumber(tt.nums)
			assert.Equal(t, actual, tt.want)

			actual2 := singleNumber2(tt.nums)
			assert.Equal(t, actual2, tt.want)
		})
	}
}
