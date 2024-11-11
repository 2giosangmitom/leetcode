package group_anagrams

import (
	"slices"
	"strings"
	"testing"

	"github.com/stretchr/testify/assert"
)

func checkEqual(t *testing.T, result, want [][]string) {
	for _, vec := range result {
		slices.Sort(vec)
	}
	for _, vec := range want {
		slices.Sort(vec)
	}
	slices.SortFunc(result, func(a, b []string) int {
		return strings.Compare(a[0], b[0])
	})
	slices.SortFunc(want, func(a, b []string) int {
		return strings.Compare(a[0], b[0])
	})

	assert.Equal(t, want, result)
}

func TestGroupAnagrams(t *testing.T) {
	cases := []struct {
		name string
		strs []string
		want [][]string
	}{
		{name: "same length", strs: []string{"eat", "tea", "tan", "ate", "nat", "bat"}, want: [][]string{{"bat"}, {"nat", "tan"}, {"ate", "eat", "tea"}}},
		{name: "empty string", strs: []string{""}, want: [][]string{{""}}},
		{name: "single character strings", strs: []string{"a", "b", "a"}, want: [][]string{{"a", "a"}, {"b"}}},
		{name: "mixed case strings", strs: []string{"a", "A"}, want: [][]string{{"a"}, {"A"}}},
		{name: "no anagrams", strs: []string{"abc", "def", "ghi"}, want: [][]string{{"abc"}, {"def"}, {"ghi"}}},
		{name: "all anagrams", strs: []string{"abc", "bca", "cab"}, want: [][]string{{"abc", "bca", "cab"}}},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			result := groupAnagrams(tt.strs)
			checkEqual(t, result, tt.want)
		})
	}
}
