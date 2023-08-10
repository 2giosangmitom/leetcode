package longestcommonprefix

import (
	"fmt"
	"testing"
)

func TestLongestCommonPrefix(t *testing.T) {
	tt := []struct {
		want string
		strs []string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := longestCommonPrefix(test.strs)
			want := test.want
			if got != want {
				t.Errorf("Got %s but want %s", got, want)
			}
		})
	}
}
