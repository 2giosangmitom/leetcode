package longest_common_prefix

import "strings"

func longestCommonPrefix(strs []string) string {
	longestPrefix := strs[0]

	for i := 1; i < len(strs); i++ {
		for strings.Index(strs[i], longestPrefix) != 0 {
			longestPrefix = longestPrefix[:len(longestPrefix)-1]
			if longestPrefix == "" {
				return ""
			}
		}
	}

	return longestPrefix
}
