package twosum

import (
	"reflect"
	"testing"
)

func Test_twoSum(t *testing.T) {
	type args struct {
		nums   []int
		target int
	}
	tests := []struct {
		name string
		args args
		want []int
	}{
		{name: "case 1", args: args{nums: []int{2, 7, 11, 15}, target: 9}, want: []int{0, 1}},
		{name: "case 2", args: args{nums: []int{3, 2, 4}, target: 6}, want: []int{1, 2}},
		{name: "case 3", args: args{nums: []int{3, 3}, target: 6}, want: []int{0, 1}},
		{name: "case 4", args: args{nums: []int{2, 3, 4, 1, 25, 8}, target: 30}, want: []int{-1}},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := twoSum(tt.args.nums, tt.args.target); !reflect.DeepEqual(got, tt.want) {
				t.Errorf("twoSum() = %v, want %v", got, tt.want)
			}
		})
	}
}

func Benchmark_twoSum(t *testing.B) {
	tests := []struct {
		nums   []int
		target int
	}{
		{nums: []int{2, 7, 11, 15}, target: 9},
		{nums: []int{3, 2, 4}, target: 6},
		{nums: []int{3, 3}, target: 6},
		{nums: []int{2, 3, 4, 1, 25, 8}, target: 30},
	}
	for _, tt := range tests {
		twoSum(tt.nums, tt.target)
	}
}
