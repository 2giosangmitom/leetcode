package longest_common_prefix

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLongestCommonPrefix(t *testing.T) {
	cases := []struct {
		name string
		strs []string
		want string
	}{
		{
			name: "common prefix exists",
			strs: []string{"flower", "flow", "flight"},
			want: "fl",
		},
		{
			name: "empty string slice",
			strs: []string{},
			want: "",
		},
		{
			name: "no common prefix",
			strs: []string{"dog", "racecar", "car"},
			want: "",
		},
		{
			name: "single string",
			strs: []string{"alone"},
			want: "alone",
		},
		{
			name: "no common prefix at all",
			strs: []string{"abc", "def", "ghi"},
			want: "",
		},
		{
			name: "same strings",
			strs: []string{"same", "same", "same"},
			want: "same",
		},
		{
			name: "prefix is the shortest string",
			strs: []string{"ab", "abc", "abcd"},
			want: "ab",
		},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := longestCommonPrefix(tt.strs)
			assert.Equal(t, actual, tt.want)
		})
	}
}
