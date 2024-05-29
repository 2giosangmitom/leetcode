package romantointeger

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRomanToInt(t *testing.T) {
	cases := []struct {
		roman string
		want  int
	}{
		{roman: "III", want: 3},
		{roman: "LVIII", want: 58},
		{roman: "MCMXCIV", want: 1994},
		{roman: "XXIV", want: 24},
		{roman: "LLVMR", want: -1},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := romanToInt(tt.roman)
			assert.Equal(t, tt.want, result)
		})
	}
}
