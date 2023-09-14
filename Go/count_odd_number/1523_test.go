package countoddnumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCountOdd(t *testing.T) {
	tests := []struct {
		low  int
		high int
		want int
	}{
		{low: 8, high: 10, want: 1},
		{low: 3, high: 6, want: 2},
		{low: 8, high: 13, want: 3},
		{low: 3, high: 7, want: 3},
		{low: 2, high: 6, want: 2},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := countOdds(tt.low, tt.high)
			result2 := countOdds2(tt.low, tt.high)
			assert.Equal(t, tt.want, result)
			assert.Equal(t, tt.want, result2)
		})
	}
}
