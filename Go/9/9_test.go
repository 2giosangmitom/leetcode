package palindrome_number

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestPalindromeNumber(t *testing.T) {
	cases := []struct {
		name string
		x    int
		want bool
	}{
		{name: "PositivePalindrome", x: 121, want: true},
		{name: "NegativeNumber", x: -121, want: false},
		{name: "SingleDigitNotPalindrome", x: 10, want: false},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := isPalindrome(tt.x)
			assert.Equal(t, actual, tt.want)
		})
	}
}
