package excelsheetcolumntitle

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_convertToTitle(t *testing.T) {
	tests := []struct {
		columnNumber int
		want         string
	}{
		{columnNumber: 1, want: "A"},
		{columnNumber: 28, want: "AB"},
		{columnNumber: 701, want: "ZY"},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := convertToTitle(tt.columnNumber)
			assert.Equal(t, tt.want, got)
		})
	}
}
