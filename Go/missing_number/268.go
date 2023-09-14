package missingnumber

func missingNumber(nums []int) int {
	sum := func(n []int) int {
		result := 0
		for _, v := range n {
			result += v
		}
		return result
	}(nums)
	total := (len(nums) * (len(nums) + 1)) / 2
	return total - sum
}
