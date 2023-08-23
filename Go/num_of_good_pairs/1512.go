package numofgoodpairs

func numIdenticalPairs(nums []int) int {
	result := 0
	numberOfCountMap := make(map[int]int)
	for _, num := range nums {
		if count, ok := numberOfCountMap[num]; ok {
			result += count
			numberOfCountMap[num] = count + 1
		} else {
			numberOfCountMap[num] = 1
		}
	}
	return result
}
