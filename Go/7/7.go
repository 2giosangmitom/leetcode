package reverse_integer

import "math"

func reverse(x int) int {
	result := 0

	for x != 0 {
		lastDigit := x % 10
		result = result*10 + lastDigit
		if result > math.MaxInt32 || result < math.MinInt32 {
			return 0
		}
		x /= 10
	}

	return result
}
