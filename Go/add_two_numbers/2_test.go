package addtwonumbers

import (
	"reflect"
	"testing"
)

func Test_addTwoNumbers(t *testing.T) {
	type args struct {
		l1 *ListNode
		l2 *ListNode
	}
	tests := []struct {
		name string
		args args
		want *ListNode
	}{
		{name: "case 1", args: args{l1: &ListNode{2, &ListNode{4, &ListNode{3, nil}}}, l2: &ListNode{5, &ListNode{6, &ListNode{4, nil}}}}, want: &ListNode{7, &ListNode{0, &ListNode{8, nil}}}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := addTwoNumbers(tt.args.l1, tt.args.l2); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("addTwoNumbers() = %v, want %v", got, tt.want)
			}
		})
	}
}
