package besttime2buynsellstock

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBestTimeToBuyAndSellStock(t *testing.T) {
	tests := []struct {
		prices []int
		want   int
	}{
		{prices: []int{7, 1, 5, 3, 6, 4}, want: 5},
		{prices: []int{7, 6, 4, 3, 1}, want: 0},
		{prices: []int{2, 1, 4}, want: 3},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := maxProfit(tt.prices)
			assert.Equal(t, tt.want, got)
		})
	}
}

func TestBestTimeToBuyAndSellStock2(t *testing.T) {
	tests := []struct {
		prices []int
		want   int
	}{
		{prices: []int{7, 1, 5, 3, 6, 4}, want: 5},
		{prices: []int{7, 6, 4, 3, 1}, want: 0},
		{prices: []int{2, 1, 4}, want: 3},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := maxProfit2(tt.prices)
			assert.Equal(t, tt.want, got)
		})
	}
}
