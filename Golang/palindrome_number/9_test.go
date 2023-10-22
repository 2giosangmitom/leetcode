package palindromenumber

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_9(t *testing.T) {
	cases := []struct {
		num  int
		want bool
	}{
		{num: -10, want: false},
		{num: 5, want: true},
		{num: 121, want: true},
		{num: 321, want: false},
		{num: 111, want: true},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			result := isPalindrome(tt.num)
			assert.Equal(t, tt.want, result)
		})
	}
}
