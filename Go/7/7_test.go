package reverse_integer

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestReverseInteger(t *testing.T) {
	cases := []struct {
		name string
		x    int
		want int
	}{
		{name: "PositiveInteger", x: 123, want: 321},
		{name: "NegativeInteger", x: -123, want: -321},
		{name: "TrailingZeros", x: 120, want: 21},
		{name: "OverflowPositive", x: 1463847412, want: 2147483641},
		{name: "OverflowNegative", x: -2147483648, want: 0},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := reverse(tt.x)
			assert.Equal(t, actual, tt.want)
		})
	}
}
