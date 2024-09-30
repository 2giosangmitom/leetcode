package valid_parentheses

import (
	"leetcode/pkg/assert"
	"testing"
)

func TestValidParentheses(t *testing.T) {
	cases := []struct {
		name string
		s    string
		want bool
	}{
		{name: "ValidSinglePair", s: "()", want: true},
		{name: "ValidMultiplePairs", s: "()[]{}", want: true},
		{name: "InvalidMixedBrackets", s: "(]", want: false},
		{name: "ValidNestedBrackets", s: "([])", want: true},
		{name: "InvalidMisplacedBrackets", s: "([}}])", want: false},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := isValid(tt.s)
			assert.Equal(t, actual, tt.want)
		})
	}
}
