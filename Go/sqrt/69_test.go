package sqrt

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_mySqrt(t *testing.T) {
	tests := []struct {
		x    int
		want int
	}{
		{x: 4, want: 2},
		{x: 8, want: 2},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := mySqrt(tt.x)
			assert.Equal(t, tt.want, got)
		})
	}
}
