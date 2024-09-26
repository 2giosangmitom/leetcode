package assert

import (
	"reflect"
	"testing"
)

func Equal(t *testing.T, actual, want any) {
	if actual != want {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", actual, actual, want, want)
	}
}

func DeepEqual(t *testing.T, actual, want any) {
	if !reflect.DeepEqual(actual, want) {
		t.Helper()
		t.Errorf("Assertion failed: got %v (%T), but want %v (%T)", actual, actual, want, want)
	}
}
