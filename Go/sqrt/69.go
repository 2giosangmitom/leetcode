package sqrt

func mySqrt(x int) int {
	left := 1
	right := x
	for left <= right {
		mid := left + (right-left)/2
		if mid == x/mid {
			return mid
		} else if mid > x/mid {
			right = mid - 1
		} else {
			left = mid + 1
		}
	}
	return left - 1
}
