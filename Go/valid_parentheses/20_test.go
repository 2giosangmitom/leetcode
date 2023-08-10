package validparentheses

import (
	"fmt"
	"testing"
)

func TestValidParentheses(t *testing.T) {
	tt := []struct {
		s    string
		want bool
	}{
		{s: "()", want: true},
		{s: "()[]{}", want: true},
		{s: "(]", want: false},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := isValid(test.s)
			want := test.want
			if got != want {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}
