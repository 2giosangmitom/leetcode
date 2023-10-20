package reverseinteger

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_7(t *testing.T) {
	cases := []struct {
		x    int
		want int
	}{
		{x: 123, want: 321},
		{x: -123, want: -321},
		{x: 120, want: 21},
		{x: 1534236469, want: 0},
		{x: -2147483648, want: 0},
		{x: 900000, want: 9},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := reverse(tt.x)
			assert.Equal(t, tt.want, result)
		})
	}
}
