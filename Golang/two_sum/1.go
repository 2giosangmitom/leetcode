package twosum

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)
	for i, num := range nums {
		complement := target - num
		if value, ok := hashMap[complement]; ok {
			return []int{value, i}
		} else {
			hashMap[num] = i
		}
	}
	return []int{-1}
}

func twoSum2(nums []int, target int) []int {
	for i, num := range nums {
		complement := target - num
		for j := i + 1; j < len(nums); j++ {
			if nums[j] == complement {
				return []int{i, j}
			}
		}
	}
	return []int{-1}
}
