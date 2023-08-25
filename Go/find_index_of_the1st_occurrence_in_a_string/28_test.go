package findindexofthe1stoccurrenceinastring

import (
	"fmt"
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_strStr(t *testing.T) {
	tests := []struct {
		haystack string
		needle   string
		want     int
	}{
		{haystack: "sadbutsad", needle: "sad", want: 0},
		{haystack: "leetcode", needle: "leeto", want: -1},
	}

	for i, tt := range tests {
		t.Run(fmt.Sprintf("case %d", i+1), func(t *testing.T) {
			got := strStr(tt.haystack, tt.needle)
			assert.Equal(t, tt.want, got)
		})
	}
}
