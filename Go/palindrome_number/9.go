/**
 * Runtime: 12ms (Beats 86.4%)
 * Memory: 4.2MB (Beats 99.4%)
 */

package palindromenumber

func isPalindrome(x int) bool {
	reverseNumber := func(n int) int {
		num := 0
		for n != 0 {
			last_num := n % 10
			num = num*10 + last_num
			n /= 10
		}
		return num
	}

	if x < 0 {
		return false
	}
	if x <= 9 && x >= 0 {
		return true
	}
	return reverseNumber(x) == x
}
