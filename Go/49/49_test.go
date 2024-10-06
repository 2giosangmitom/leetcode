package group_anagrams

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestGroupAnagrams(t *testing.T) {
	cases := []struct {
		name string
		strs []string
		want [][]string
	}{
		{name: "same length", strs: []string{"eat", "tea", "tan", "ate", "nat", "bat"}, want: [][]string{{"nat", "tan"}, {"bat"}, {"ate", "eat", "tea"}}},
		{name: "empty string", strs: []string{""}, want: [][]string{{""}}},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			result := groupAnagrams(tt.strs)
			assert.SliceUnorderedEqual(t, result, tt.want)
		})
	}
}
