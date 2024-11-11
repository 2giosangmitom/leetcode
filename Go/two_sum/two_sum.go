package two_sum

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)
	for i := 0; i < len(nums); i++ {
		remainder := target - nums[i]
		if value, ok := hashMap[remainder]; !ok {
			hashMap[nums[i]] = i
		} else {
			return []int{value, i}
		}
	}
	return []int{}
}
