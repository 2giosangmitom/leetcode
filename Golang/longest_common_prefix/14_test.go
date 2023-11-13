package longest_common_prefix

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_14(t *testing.T) {
	cases := []struct {
		strs []string
		want string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
		{strs: []string{"chi", "chien", "chau"}, want: "ch"},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := longestCommonPrefix(tt.strs)
			assert.Equal(t, tt.want, result)
		})
	}
}
