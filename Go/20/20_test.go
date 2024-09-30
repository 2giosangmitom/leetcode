package valid_parentheses

import (
	"fmt"
	"leetcode/pkg/assert"
	"testing"
)

func Test_20_Valid_Parentheses(t *testing.T) {
	cases := []struct {
		s    string
		want bool
	}{
		{s: "()", want: true},
		{s: "()[]{}", want: true},
		{s: "(]", want: false},
		{s: "([])", want: true},
		{s: "([}}])", want: false},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("cases %d", i+1), func(t *testing.T) {
			actual := isValid(tt.s)
			assert.Equal(t, actual, tt.want)
		})
	}
}
