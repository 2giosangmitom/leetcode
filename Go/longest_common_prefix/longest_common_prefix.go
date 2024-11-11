package longest_common_prefix

import "strings"

func longestCommonPrefix(strs []string) string {
	longestPrefix := (func(strs []string) string {
		if len(strs) == 0 {
			return ""
		}
		return strs[0]
	})(strs)

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
