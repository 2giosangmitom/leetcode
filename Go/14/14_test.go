package longest_common_prefix

import (
	"fmt"
	"leetcode/pkg/assert"
	"testing"
)

func Test_14_Longest_Common_Prefix(t *testing.T) {
	cases := []struct {
		strs []string
		want string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			actual := longestCommonPrefix(tt.strs)
			assert.Equal(t, actual, tt.want)
		})
	}
}
