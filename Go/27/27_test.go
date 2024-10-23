package remove_elements

import (
	"fmt"
	"leetcode/pkg/assert"
	"slices"
	"testing"
)

func TestRemoveElements(t *testing.T) {
	cases := []struct {
		nums     []int
		val      int
		wantNums []int
		want     int
	}{
		{nums: []int{3, 2, 2, 3}, val: 3, want: 2, wantNums: []int{2, 2}},
		{nums: []int{0, 1, 2, 2, 3, 0, 4, 2}, val: 2, want: 5, wantNums: []int{0, 0, 1, 3, 4}},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			nums := tt.nums
			result := removeElement(nums, tt.val)

			assert.Equal(t, tt.want, result)
			postArr := nums[0:len(tt.wantNums)]
			slices.Sort(postArr)
			assert.DeepEqual(t, postArr, tt.wantNums)
		})
	}
}
