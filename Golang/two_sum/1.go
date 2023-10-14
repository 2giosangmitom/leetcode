package twosum

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)
	for i, num := range nums {
		complement := target - num
		if _, ok := hashMap[complement]; ok {
			return []int{hashMap[complement], i}
		} else {
			hashMap[num] = i
		}
	}
	return []int{-1}
}
