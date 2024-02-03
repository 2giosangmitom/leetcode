package singlenumber

func singleNumber(nums []int) int {
	hash_map := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		if value, ok := hash_map[nums[i]]; ok {
			hash_map[nums[i]] = value + 1
		} else {
			hash_map[nums[i]] = 1
		}
	}

	result := 0
	for i := 0; i < len(nums); i++ {
		if hash_map[nums[i]] == 1 {
			result = nums[i]
		}
	}

	return result
}
