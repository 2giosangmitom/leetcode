package reverse_integer

import (
	"fmt"
	"leetcode/pkg/assert"
	"testing"
)

func Test_7_Reverse_Integer(t *testing.T) {
	cases := []struct {
		x    int
		want int
	}{
		{x: 123, want: 321},
		{x: -123, want: -321},
		{x: 120, want: 21},
		{x: 1463847412, want: 2147483641},
		{x: -2147483648, want: 0},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			actual := reverse(tt.x)
			assert.Equal(t, actual, tt.want)
		})
	}
}
