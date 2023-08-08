package main

import (
	"fmt"
	"reflect"
	"testing"
)

func TestTwoSum(t *testing.T) {
	tt := []struct {
		nums   []int
		want   []int
		target int
	}{
		{nums: []int{2, 7, 11, 15}, target: 9, want: []int{0, 1}},
		{nums: []int{3, 2, 4}, target: 6, want: []int{1, 2}},
		{nums: []int{3, 3}, target: 6, want: []int{0, 1}},
		{nums: []int{2, 3, 4, 1, 25, 8}, target: 30, want: []int{-1}},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := twoSum(test.nums, test.target)
			want := test.want
			if !reflect.DeepEqual(got, want) {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}
