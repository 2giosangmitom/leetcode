package lenoflastword

import (
	"strings"
)

func lengthOfLastWord(s string) int {
	count := 0
	s = strings.TrimSpace(s)
	for i := len(s) - 1; i >= 0 && string(s[i]) != " "; i-- {
		count++
	}
	return count
}
