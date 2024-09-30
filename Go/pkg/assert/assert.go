package assert

import (
	"reflect"
	"testing"
)

// Equal checks if actual and want are equal. If they are not, it reports an error with a detailed message.
func Equal(t *testing.T, actual, want any) {
	if actual != want {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", actual, actual, want, want)
	}
}

// DeepEqual checks if actual and want are deeply equal using reflect.DeepEqual.
// If they are not, it reports an error with a detailed message.
func DeepEqual(t *testing.T, actual, want any) {
	if !reflect.DeepEqual(actual, want) {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", actual, actual, want, want)
	}
}
