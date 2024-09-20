package assert

import (
	"reflect"
	"testing"
)

// Equal compares two values `result` and `want` for deep equality
func Equal(t *testing.T, result, want any) {
	if !reflect.DeepEqual(result, want) {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", result, result, want, want)
	}
}
