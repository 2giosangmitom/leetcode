package uniqueemailaddresses

import "testing"

func Test_numUniqueEmails(t *testing.T) {
	type args struct {
		emails []string
	}
	tests := []struct {
		name string
		args args
		want int
	}{
		{name: "case 1", args: args{[]string{"test.email+alex@leetcode.com", "test.e.mail+bob.cathy@leetcode.com", "testemail+david@lee.tcode.com"}}, want: 2},
		{name: "case 2", args: args{[]string{"a@leetcode.com", "b@leetcode.com", "c@leetcode.com"}}, want: 3},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := numUniqueEmails(tt.args.emails); got != tt.want {
				t.Errorf("numUniqueEmails() = %v, want %v", got, tt.want)
			}
		})
	}
}
