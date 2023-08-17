package reverseinteger

import (
	"fmt"
	"testing"
)

func TestReverse(t *testing.T) {
	tt := [...]struct {
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

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := reverse(test.x)
			if got != test.want {
				t.Errorf("Got %d but want %d", got, test.want)
			}
		})
	}
}
