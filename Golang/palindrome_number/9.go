package palindromenumber

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	reverse := func(n int) int {
		result := 0
		for n != 0 {
			lastDigit := n % 10
			result = result*10 + lastDigit
			n /= 10
		}
		return result
	}

	return reverse(x) == x
}
