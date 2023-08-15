package missingnumber

import (
	"fmt"
	"testing"
)

func TestMissingNumber(t *testing.T) {
	tt := [...]struct {
		nums []int
		want int
	}{
		{nums: []int{3, 0, 1}, want: 2},
		{nums: []int{0, 1}, want: 2},
		{nums: []int{9, 6, 4, 2, 3, 5, 7, 0, 1}, want: 8},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := missingNumber(test.nums)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
