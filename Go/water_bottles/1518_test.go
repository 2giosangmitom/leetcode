package waterbottles

import (
	"fmt"
	"testing"
)

func TestWaterBottles(t *testing.T) {
	tt := []struct {
		numBottles  int
		numExchange int
		want        int
	}{
		{numBottles: 9, numExchange: 3, want: 13},
		{numBottles: 15, numExchange: 4, want: 19},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := numWaterBottles(test.numBottles, test.numExchange)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
