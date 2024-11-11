package remove_elements

import (
	"slices"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRemoveElements(t *testing.T) {
	cases := []struct {
		name     string
		nums     []int
		val      int
		wantNums []int
		want     int
	}{
		{name: "continuous occurrences", nums: []int{3, 2, 2, 3}, val: 3, want: 2, wantNums: []int{2, 2}},
		{name: "multiple occurrences", nums: []int{0, 1, 2, 2, 3, 0, 4, 2}, val: 2, want: 5, wantNums: []int{0, 0, 1, 3, 4}},
		{name: "no occurrences", nums: []int{1, 2, 3, 4, 5}, val: 6, want: 5, wantNums: []int{1, 2, 3, 4, 5}},
		{name: "all occurrences", nums: []int{2, 2, 2, 2}, val: 2, want: 0, wantNums: []int{}},
		{name: "mixed occurrences", nums: []int{4, 5, 4, 6, 4, 7}, val: 4, want: 3, wantNums: []int{5, 6, 7}},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			nums := tt.nums
			result := removeElement(nums, tt.val)

			assert.Equal(t, tt.want, result)

			postArr := nums[0:len(tt.wantNums)]
			slices.Sort(postArr)
			assert.Equal(t, postArr, tt.wantNums)
		})
	}
}
