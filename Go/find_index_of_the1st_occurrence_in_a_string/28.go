package findindexofthe1stoccurrenceinastring

func strStr(haystack string, needle string) int {
	needleLength := len(needle)
	haystackLength := len(haystack)
	if needleLength > haystackLength {
		return -1
	}
	for i := 0; i <= haystackLength-needleLength; i++ {
		if haystack[i:i+needleLength] == needle {
			return i
		}
	}
	return -1
}
