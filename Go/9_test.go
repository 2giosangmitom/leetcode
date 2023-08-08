package main

import "testing"

func TestPalindromeNum(t *testing.T) {
	tt := []struct {
		name string
		num  int
		want bool
	}{
		{name: "case 1", num: -10, want: false},
		{name: "case 2", num: 5, want: true},
		{name: "case 3", num: 121, want: true},
	}

	for _, test := range tt {
		t.Run(test.name, func(t *testing.T) {
			got := isPalindrome(test.num)
			want := test.want
			if got != want {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}
