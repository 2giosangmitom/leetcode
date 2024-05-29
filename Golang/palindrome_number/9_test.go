package palindromenumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsPalindrome(t *testing.T) {
	cases := []struct {
		num  int
		want bool
	}{
		{num: -10, want: false},
		{num: 5, want: true},
		{num: 121, want: true},
		{num: 321, want: false},
		{num: 111, want: true},
		{num: 12321, want: true},
		{num: 123321, want: true},
		{num: 1001, want: true},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := isPalindrome(tt.num)
			assert.Equal(t, tt.want, result)
		})
	}
}
