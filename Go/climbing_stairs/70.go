package climbingstairs

func climbStairs(n int) int {
	if n == 1 || n == 2 {
		return n
	}
	a := 1
	b := 2
	for i := 3; i < n; i++ {
		temp := b
		b += a
		a = temp
	}
	return a + b
}
