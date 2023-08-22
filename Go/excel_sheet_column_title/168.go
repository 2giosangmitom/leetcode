/**
 * Runtime: 1ms (Beats 74.84%)
 * Memory: 1.87MB (Beats 94.34%)
 */

package excelsheetcolumntitle

func convertToTitle(columnNumber int) string {
	result := []byte{}
	for columnNumber > 0 {
		columnNumber -= 1
		result = append(result, byte('A')+(byte(columnNumber%26)))
		columnNumber /= 26
	}
	result = func(s []byte) []byte {
		r := []byte{}
		for i := len(s) - 1; i >= 0; i-- {
			r = append(r, s[i])
		}
		return r
	}(result)
	return string(result)
}
