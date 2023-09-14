package validparentheses

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_isValid(t *testing.T) {
	tests := []struct {
		s    string
		want bool
	}{
		{s: "()", want: true},
		{s: "()[]{}", want: true},
		{s: "(]", want: false},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := isValid(tt.s)
			assert.Equal(t, tt.want, got)
		})
	}
}
