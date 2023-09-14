package plusone

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_PlusOne(t *testing.T) {
	tests := []struct {
		digits []int
		want   []int
	}{
		{digits: []int{1, 2, 3}, want: []int{1, 2, 4}},
		{digits: []int{4, 3, 2, 1}, want: []int{4, 3, 2, 2}},
		{digits: []int{9}, want: []int{1, 0}},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			result := plusOne(tt.digits)
			assert.Equal(t, tt.want, result)
		})
	}
}
