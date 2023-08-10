/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2.1MB (Beats 33.99%)
 */

package lenoflastword

import (
	"strings"
)

func lengthOfLastWord(s string) int {
	count := 0
	s = strings.TrimSpace(s)
	for i := len(s) - 1; i >= 0; i-- {
		if string(s[i]) == " " {
			break
		}
		count++
	}
	return count
}
