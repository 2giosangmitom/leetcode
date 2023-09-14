package palindromenumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPalindromeNum(t *testing.T) {
	tests := []struct {
		num  int
		want bool
	}{
		{num: -10, want: false},
		{num: 5, want: true},
		{num: 121, want: true},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := isPalindrome(tt.num)
			assert.Equal(t, tt.want, got)
		})
	}
}
