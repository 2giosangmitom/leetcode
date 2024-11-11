package single_number

import "slices"

func singleNumber(nums []int) int {
	result := 0
	for _, num := range nums {
		result ^= num
	}
	return result
}

func singleNumber2(nums []int) int {
	slices.Sort(nums)
	for i := 1; i < len(nums); i += 2 {
		if nums[i] != nums[i-1] {
			return nums[i-1]
		}
	}
	return nums[len(nums)-1]
}
