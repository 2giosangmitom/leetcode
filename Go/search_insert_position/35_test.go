package searchinsertposition

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_searchInsert(t *testing.T) {
	tests := []struct {
		nums   []int
		target int
		want   int
	}{
		{nums: []int{1, 3, 5, 6}, target: 5, want: 2},
		{nums: []int{1, 3, 5, 6}, target: 2, want: 1},
		{nums: []int{1, 3, 5, 6}, target: 7, want: 4},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := searchInsert(tt.nums, tt.target)
			assert.Equal(t, tt.want, got)
		})
	}
}

func Benchmark_searchInsert(b *testing.B) {
	type args struct {
		nums   []int
		target int
	}
	tests := []struct {
		name string
		args args
	}{
		{name: "case 1", args: args{nums: []int{1, 3, 5, 6}, target: 5}},
		{name: "case 2", args: args{nums: []int{1, 3, 5, 6}, target: 2}},
		{name: "case 3", args: args{nums: []int{1, 3, 5, 6}, target: 7}},
	}
	for _, tt := range tests {
		searchInsert(tt.args.nums, tt.args.target)
	}
}
