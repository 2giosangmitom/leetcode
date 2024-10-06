package assert

import (
	"errors"
	"fmt"
	"reflect"
	"testing"
)

// Equal checks if actual and want are equal. If they are not, it reports an error with a detailed message.
func Equal(t *testing.T, actual, want any) {
	if actual != want {
		t.Helper()
		t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, want)
	}
}

// DeepEqual checks if actual and want are deeply equal using reflect.DeepEqual.
// If they are not, it reports an error with a detailed message.
func DeepEqual(t *testing.T, actual, want any) {
	if !reflect.DeepEqual(actual, want) {
		t.Helper()
		t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, want)
	}
}

// True checks if actual is true. If not, it reports an error.
func True(t *testing.T, actual bool) {
	if !actual {
		t.Helper()
		t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, true)
	}
}

// unorderedCompare checks if two slices contain the same elements, regardless of order.
func unorderedCompare[T comparable](left, right []T) bool {
	if len(left) != len(right) {
		return false
	}

	// Count occurrences of each element in both slices.
	leftCount := make(map[T]int)
	rightCount := make(map[T]int)

	for _, v := range left {
		leftCount[v]++
	}
	for _, v := range right {
		rightCount[v]++
	}

	return reflect.DeepEqual(leftCount, rightCount)
}

// SliceUnorderedEqual checks if actual and want slices are equal regardless of order.
// If they are not, it reports an error with a detailed message.
func SliceUnorderedEqual(t *testing.T, actual, want any) {
	if reflect.TypeOf(actual) != reflect.TypeOf(want) {
		panic(errors.New("actual and want must have the same type"))
	}

	switch v := actual.(type) {
	case []string:
		if wSlice, ok := want.([]string); ok {
			if !unorderedCompare(v, wSlice) {
				t.Helper()
				t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, want)
			}
		}

	case []int:
		if wSlice, ok := want.([]int); ok {
			if !unorderedCompare(v, wSlice) {
				t.Helper()
				t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, want)
			}
		}

	case [][]string:
		if wSlice, ok := want.([][]string); ok {
			if len(v) != len(wSlice) {
				t.Helper()
				t.Errorf("Length mismatch: got %d, want %d", len(v), len(wSlice))
			} else {
				for _, slice := range v {
					found := false
					for _, slice2 := range wSlice {
						if unorderedCompare(slice, slice2) {
							found = true
							break
						}
					}
					if !found {
						t.Helper()
						t.Errorf("Assertion failed\n Got: %v\n Want: %v", actual, want)
						break
					}
				}
			}
		} else {
			panic(fmt.Errorf("unexpected type for want: %T", want))
		}

	default:
		panic(fmt.Errorf("type %T is not supported", v))
	}
}
