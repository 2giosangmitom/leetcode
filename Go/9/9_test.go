package palindrome_number

import (
	"fmt"
	"leetcode/pkg/assert"
	"testing"
)

func Test_9_Palindrome_Number(t *testing.T) {
	cases := []struct {
		x    int
		want bool
	}{
		{x: 121, want: true},
		{x: -121, want: false},
		{x: 10, want: false},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			actual := isPalindrome(tt.x)
			assert.Equal(t, actual, tt.want)
		})
	}
}
