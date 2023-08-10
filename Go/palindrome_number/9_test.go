package palindromenumber

import (
	"fmt"
	"testing"
)

func TestPalindromeNum(t *testing.T) {
	tt := []struct {
		num  int
		want bool
	}{
		{num: -10, want: false},
		{num: 5, want: true},
		{num: 121, want: true},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := isPalindrome(test.num)
			want := test.want
			if got != want {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}
