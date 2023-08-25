package longestcommonprefix

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLongestCommonPrefix(t *testing.T) {
	tests := [...]struct {
		want string
		strs []string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := longestCommonPrefix(tt.strs)
			assert.Equal(t, tt.want, got)
		})
	}
}
