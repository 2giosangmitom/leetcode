package reverseinteger

/**
 * Runtime: 0ms (Beats 100%)
 * Memory: 2.1MB (Beats 98.24%)
 */

import "math"

func reverse(x int) int {
	result := 0
	for x != 0 {
		last_number := x % 10
		result = result*10 + last_number
		x /= 10
	}
	if result > math.MaxInt32 || result < math.MinInt32 {
		return 0
	}
	return result
}
