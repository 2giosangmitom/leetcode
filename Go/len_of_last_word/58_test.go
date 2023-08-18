package lenoflastword

import (
	"fmt"
	"testing"
)

func TestLenOfLastWord(t *testing.T) {
	tt := [...]struct {
		s    string
		want int
	}{
		{s: "Hello World", want: 5},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := lengthOfLastWord(test.s)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}

func BenchmarkLenOfLastWord(t *testing.B) {
	tt := []struct {
		s string
	}{
		{s: "Hello World"},
	}

	for _, test := range tt {
		lengthOfLastWord(test.s)
	}
}
