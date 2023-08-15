package sqrt

import (
	"fmt"
	"testing"
)

func TestSqrt(t *testing.T) {
	tt := [...]struct {
		x    int
		want int
	}{
		{x: 4, want: 2},
		{x: 8, want: 2},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := mySqrt(test.x)
			if got != test.want {
				t.Errorf("Got %d but want %d", got, test.want)
			}
		})
	}
}
