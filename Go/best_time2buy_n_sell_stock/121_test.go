package besttime2buynsellstock

import (
	"fmt"
	"testing"
)

func TestBestTimeToBuyAndSellStock(t *testing.T) {
	tt := [...]struct {
		prices []int
		want   int
	}{
		{prices: []int{7, 1, 5, 3, 6, 4}, want: 5},
		{prices: []int{7, 6, 4, 3, 1}, want: 0},
		{prices: []int{2, 1, 4}, want: 3},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := maxProfit(test.prices)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
