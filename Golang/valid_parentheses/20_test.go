package validparentheses

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestIsValid(t *testing.T) {
	cases := []struct {
		s    string
		want bool
	}{
		{s: "()", want: true},
		{s: "()[]{}", want: true},
		{s: "(]", want: false},
		{s: "){", want: false},
		{s: "][]", want: false},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := isValid(tt.s)
			assert.Equal(t, tt.want, result)
		})
	}
}
