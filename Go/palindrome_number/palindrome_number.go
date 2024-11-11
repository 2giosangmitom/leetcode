package palindrome_number

func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}

	temp := x
	reversed := 0

	for temp != 0 {
		lastDigit := temp % 10
		reversed = reversed*10 + lastDigit
		temp /= 10
	}

	return reversed == x
}
