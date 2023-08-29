package binarysearch

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBinarySearch(t *testing.T) {
	tests := []struct {
		nums   []int
		target int
		want   int
	}{
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 9, want: 4},
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 2, want: -1},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := search(tt.nums, tt.target)
			assert.Equal(t, tt.want, got)
		})
	}
}

func TestBinarySearch2(t *testing.T) {
	tests := [...]struct {
		nums   []int
		target int
		want   int
	}{
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 9, want: 4},
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 2, want: -1},
		{nums: []int{3, 4, 14, 51, 66, 102}, target: 66, want: 4},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := search2(tt.nums, tt.target)
			assert.Equal(t, tt.want, got)
		})
	}
}
