package add_binary

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestAddBinary(t *testing.T) {
	cases := []struct {
		a    string
		b    string
		want string
	}{
		{a: "11", b: "1", want: "100"},
		{a: "1010", b: "1011", want: "10101"},
		{a: "0", b: "0", want: "0"},
	}

	for i, tt := range cases {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			actual := addBinary(tt.a, tt.b)
			assert.Equal(t, tt.want, actual)
		})
	}
}
