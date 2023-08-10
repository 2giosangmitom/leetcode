package roman2int

import (
	"fmt"
	"testing"
)

func TestRoman2Int(t *testing.T) {
	tt := []struct {
		roman string
		want  int
	}{
		{roman: "III", want: 3},
		{roman: "LVIII", want: 58},
		{roman: "MCMXCIV", want: 1994},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := romanToInt(test.roman)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
