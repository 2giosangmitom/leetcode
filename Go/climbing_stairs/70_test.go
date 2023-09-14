package climbingstairs

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestClimbingStairs(t *testing.T) {
	tests := []struct {
		n    int
		want int
	}{
		{n: 2, want: 2},
		{n: 3, want: 3},
		{n: 4, want: 5},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := climbStairs(tt.n)
			assert.Equal(t, tt.want, got)
		})
	}
}
