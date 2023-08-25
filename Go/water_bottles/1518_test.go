package waterbottles

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_numWaterBottles(t *testing.T) {
	tests := []struct {
		numBottles  int
		numExchange int
		want        int
	}{
		{numBottles: 9, numExchange: 3, want: 13},
		{numBottles: 15, numExchange: 4, want: 19},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := numWaterBottles(tt.numBottles, tt.numExchange)
			assert.Equal(t, tt.want, got)
		})
	}
}
