package main

import (
	"fmt"
	"reflect"
	"testing"
)

// 1. Two Sum
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

// 9. Palindrome Number
func TestPalindromeNum(t *testing.T) {
	tt := []struct {
		num  int
		want bool
	}{
		{num: -10, want: false},
		{num: 5, want: true},
		{num: 121, want: true},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := isPalindrome(test.num)
			want := test.want
			if got != want {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}

// 13. Roman To Integer
func TestRoman2Int(t *testing.T) {
	tt := []struct {
		roman string
		want  int
	}{
		{roman: "III", want: 3},
		{roman: "LVIII", want: 58},
		{roman: "MCMXCIV", want: 1994},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := romanToInt(test.roman)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}

// 14. Longest Common Prefix
func TestLongestCommonPrefix(t *testing.T) {
	tt := []struct {
		want string
		strs []string
	}{
		{strs: []string{"flower", "flow", "flight"}, want: "fl"},
		{strs: []string{"dog", "racecar", "car"}, want: ""},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := longestCommonPrefix(test.strs)
			want := test.want
			if got != want {
				t.Errorf("Got %s but want %s", got, want)
			}
		})
	}
}

// 20. Valid Parentheses
func TestValidParentheses(t *testing.T) {
	tt := []struct {
		s    string
		want bool
	}{
		{s: "()", want: true},
		{s: "()[]{}", want: true},
		{s: "(]", want: false},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := isValid(test.s)
			want := test.want
			if got != want {
				t.Errorf("Got %v but want %v", got, want)
			}
		})
	}
}

// 704. Binary Search
func TestBinarySearch(t *testing.T) {
	tt := []struct {
		nums   []int
		target int
		want   int
	}{
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 9, want: 4},
		{nums: []int{-1, 0, 3, 5, 9, 12}, target: 2, want: -1},
	}

	for i, test := range tt {
		t.Run(fmt.Sprintf("case %d", i), func(t *testing.T) {
			got := search(test.nums, test.target)
			want := test.want
			if got != want {
				t.Errorf("Got %d but want %d", got, want)
			}
		})
	}
}
