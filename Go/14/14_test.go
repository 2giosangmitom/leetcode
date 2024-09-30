package longest_common_prefix

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestLongestCommonPrefix(t *testing.T) {
	cases := []struct {
		name string
		strs []string
		want string
	}{
		{
			name: "CommonPrefixInMultipleWords",
			strs: []string{"flower", "flow", "flight"},
			want: "fl",
		},
		{
			name: "NoCommonPrefix",
			strs: []string{"dog", "racecar", "car"},
			want: "",
		},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := longestCommonPrefix(tt.strs)
			assert.Equal(t, actual, tt.want)
		})
	}
}
