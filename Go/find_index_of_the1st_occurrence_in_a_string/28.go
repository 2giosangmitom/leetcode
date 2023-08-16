/**
 * Runtime: 1ms (Beats 72.9%)
 * Memory: 2MB (Beats 50.55%)
 */

package findindexofthe1stoccurrenceinastring

func strStr(haystack string, needle string) int {
	needleLength := len(needle)
	haystackLength := len(haystack)
	if needleLength > haystackLength {
		return -1
	}
	for i := 0; i <= haystackLength-needleLength; i++ {
		if haystack[i] == needle[0] {
			j := 0
			for j < needleLength && haystack[i+j] == needle[j] {
				j++
			}
			if j == needleLength {
				return i
			}
		}
	}
	return -1
}
