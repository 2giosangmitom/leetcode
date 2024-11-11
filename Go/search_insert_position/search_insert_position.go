package search_insert_position

func searchInsert(nums []int, target int) int {
	start, end := 0, len(nums)-1

	for start <= end {
		middle := (end-start)/2 + start
		if nums[middle] == target {
			return middle
		}

		if nums[middle] < target {
			start = middle + 1
		} else {
			end = middle - 1
		}
	}

	return start
}
