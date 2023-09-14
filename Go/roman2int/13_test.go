package roman2int

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_romanToInt(t *testing.T) {
	tests := []struct {
		s    string
		want int
	}{
		{s: "III", want: 3},
		{s: "LVIII", want: 58},
		{s: "MCMXCIV", want: 1994},
	}
	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := romanToInt(tt.s)
			assert.Equal(t, tt.want, got)
		})
	}
}
