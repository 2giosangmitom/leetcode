package longest_common_prefix

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLongestCommonPrefix(t *testing.T) {
	cases := []struct {
		strs []string
		want string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
		{strs: []string{"chi", "chien", "chau"}, want: "ch"},
		{strs: []string{"", "", ""}, want: ""},
		{strs: []string{"apple", "apricot", "ape"}, want: "ap"},
		{strs: []string{"", "123", "ab"}, want: ""},
		{strs: []string{"race", "racer", "races"}, want: "race"},
	}

	for i, tc := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := longestCommonPrefix(tc.strs)
			assert.Equal(t, tc.want, result, fmt.Sprintf("test case %d failed", i))
		})
	}
}
