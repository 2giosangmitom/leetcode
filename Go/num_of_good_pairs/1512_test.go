package numofgoodpairs

import (
	"fmt"
	"testing"
)

func TestNumOfGoodPairs(t *testing.T) {
	tt := []struct {
		nums []int
		want int
	}{
		{nums: []int{1, 2, 3, 1, 1, 3}, want: 4},
		{nums: []int{1, 1, 1, 1}, want: 6},
		{nums: []int{1, 2, 3}, want: 0},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := numIdenticalPairs(test.nums)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
