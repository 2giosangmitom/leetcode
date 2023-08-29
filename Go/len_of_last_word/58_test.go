package lenoflastword

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLenOfLastWord(t *testing.T) {
	tests := [...]struct {
		s    string
		want int
	}{
		{s: "Hello World", want: 5},
		{s: "   fly me   to   the moon  ", want: 4},
		{s: "luffy is still joyboy", want: 6},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := lengthOfLastWord(tt.s)
			assert.Equal(t, tt.want, got)
		})
	}
}
