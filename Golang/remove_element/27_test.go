package remove_element

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func Test_RemoveElement(t *testing.T) {
	cases := []struct {
		nums     []int
		val      int
		want     int
		wantNums []int
	}{
		{nums: []int{3, 2, 2, 3}, val: 3, want: 2, wantNums: []int{2, 2}},
		{nums: []int{0, 1, 2, 2, 3, 0, 4, 2}, val: 2, want: 5, wantNums: []int{0, 1, 3, 0, 4}},
	}

	for i, tc := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			nums := tc.nums
			result := removeElement(nums, tc.val)
			assert.Equal(t, tc.want, result)
			assert.Equal(t, tc.wantNums, nums[0:len(tc.wantNums)])
		})
	}
}
