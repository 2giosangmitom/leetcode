package valid_parentheses

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestValidParentheses(t *testing.T) {
	cases := []struct {
		name string
		s    string
		want bool
	}{
		{name: "valid simple parentheses", s: "()", want: true},
		{name: "valid multiple types", s: "()[]{}", want: true},
		{name: "invalid mixed parentheses", s: "(]", want: false},
		{name: "valid nested parentheses", s: "([])", want: true},
		{name: "invalid nested parentheses", s: "([}}])", want: false},
		{name: "invalid single opening bracket", s: "(", want: false},
		{name: "invalid single closing bracket", s: ")", want: false},
	}

	for _, tt := range cases {
		t.Run(tt.name, func(t *testing.T) {
			actual := isValid(tt.s)
			assert.Equal(t, actual, tt.want)
		})
	}
}
