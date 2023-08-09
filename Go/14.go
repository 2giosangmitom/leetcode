/**
 * Runtime: 2ms (Beats 63.36%)
 * Memory: 2.4MB (Beats 31.26%)
 */

package main

import "strings"

func longestCommonPrefix(strs []string) string {
	prefix := strs[0]
	for i := 1; i < len(strs); i++ {
		for strings.Index(strs[i], prefix) != 0 {
			prefix = prefix[:len(prefix)-1]
			if len(prefix) == 0 {
				return ""
			}
		}
	}
	return prefix
}
