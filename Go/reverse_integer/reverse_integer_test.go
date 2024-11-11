package reverse_integer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReverseInteger(t *testing.T) {
	cases := []struct {
		name string
		x    int
		want int
	}{
		{name: "positive integer", x: 123, want: 321},
		{name: "negative integer", x: -123, want: -321},
		{name: "integer with trailing zeros", x: 120, want: 21},
		{name: "encounter overflow", x: -2147483648, want: 0},
		{name: "encounter overflow", x: 2147483647, want: 0},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := reverse(tt.x)
			assert.Equal(t, actual, tt.want)
		})
	}
}
