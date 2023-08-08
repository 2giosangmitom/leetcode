package main

import (
	"reflect"
	"testing"
)

func TestTwoSum(t *testing.T) {
	tt := []struct {
		name   string
		nums   []int
		want   []int
		target int
	}{
		{name: "case 1", nums: []int{2, 7, 11, 15}, target: 9, want: []int{0, 1}},
		{name: "case 2", nums: []int{3, 2, 4}, target: 6, want: []int{1, 2}},
		{name: "case 3", nums: []int{3, 3}, target: 6, want: []int{0, 1}},
		{name: "case 4", nums: []int{2, 3, 4, 1, 25, 8}, target: 30, want: []int{-1}},
	}

	for _, test := range tt {
		t.Run(test.name, func(t *testing.T) {
			got := twoSum(test.nums, test.target)
			want := test.want
			if !reflect.DeepEqual(got, want) {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}
