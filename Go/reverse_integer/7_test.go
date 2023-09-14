package reverseinteger

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestReverse(t *testing.T) {
	tests := []struct {
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

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := reverse(tt.x)
			assert.Equal(t, tt.want, got)
		})
	}
}
