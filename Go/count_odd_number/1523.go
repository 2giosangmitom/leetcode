package countoddnumber

func countOdds(low int, high int) int {
	count := 0
	for i := low; i <= high; i++ {
		if i%2 != 0 {
			count++
		}
	}
	return count
}

// Second solution
func countOdds2(low int, high int) int {
	length := high - low + 1
	result := length / 2
	if low%2 == 1 && high%2 == 1 {
		result++
	}
	return result
}
