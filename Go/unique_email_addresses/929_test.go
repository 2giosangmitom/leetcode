package uniqueemailaddresses

import (
	"fmt"
	"testing"
)

func TestUniqueEmailAddresses(t *testing.T) {
	tt := [...]struct {
		emails []string
		want   int
	}{
		{emails: []string{"test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"}, want: 2},
		{emails: []string{"a@leetcode.com", "b@leetcode.com", "c@leetcode.com"}, want: 3},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := numUniqueEmails(test.emails)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
