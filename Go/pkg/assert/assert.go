package assert

import (
	"reflect"
	"testing"
)

// Equal compares two values `actual` and `want` for deep equality
func Equal(t *testing.T, actual, want any) {
	if !reflect.DeepEqual(actual, want) {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", actual, actual, want, want)
	}
}
