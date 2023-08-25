package uniqueemailaddresses

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_numUniqueEmails(t *testing.T) {
	tests := []struct {
		emails []string
		want   int
	}{
		{
			emails: []string{"test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"},
			want:   2,
		},
		{
			emails: []string{"a@leetcode.com", "b@leetcode.com", "c@leetcode.com"},
			want:   3,
		},
	}
	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := numUniqueEmails(tt.emails)
			assert.Equal(t, tt.want, got)
		})
	}
}
