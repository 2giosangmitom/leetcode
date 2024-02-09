package summary_ranges

import "fmt"

func summaryRanges(nums []int) []string {
	result := []string{}

	for i := 0; i < len(nums); i++ {
		end := i
		for end+1 < len(nums) && nums[end+1]-nums[end] == 1 {
			end++
		}

		if end > i {
			result = append(result, fmt.Sprintf("%d->%d", nums[i], nums[end]))
		} else {
			result = append(result, fmt.Sprintf("%d", nums[end]))
		}

		i = end
	}

	return result
}
