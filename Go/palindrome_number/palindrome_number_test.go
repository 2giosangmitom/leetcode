package palindrome_number

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_palindrome_number(t *testing.T) {
	cases := []struct {
		name string
		x    int
		want bool
	}{
		{name: "positive palindrome", x: 121, want: true},
		{name: "negative palindrome", x: -121, want: false},
		{name: "not palindrome ending in zero", x: 10, want: false},
		{name: "not palindrome large number", x: 1234567899, want: false},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := isPalindrome(tt.x)
			assert.Equal(t, tt.want, actual)
		})
	}
}
