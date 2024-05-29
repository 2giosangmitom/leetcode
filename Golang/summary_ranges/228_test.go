package summary_ranges

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSummaryRanges(t *testing.T) {
	cases := []struct {
		nums []int
		want []string
	}{
		{nums: []int{0, 1, 2, 4, 5, 7}, want: []string{"0->2", "4->5", "7"}},
		{nums: []int{0, 2, 3, 4, 6, 8, 9}, want: []string{"0", "2->4", "6", "8->9"}},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := summaryRanges(tt.nums)
			assert.Equal(t, tt.want, result)
		})
	}
}
