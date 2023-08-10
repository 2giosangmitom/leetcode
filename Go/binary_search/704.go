/**
 * Runtime: 28ms (Beats 93.1%)
 * Memory: 6.9MB (Beats 71.29%)
 */

package binarysearch

func search(nums []int, target int) int {
	left := 0
	right := len(nums) - 1
	for left <= right {
		mid := left + (right-left)/2
		if nums[mid] == target {
			return mid
		} else if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}

// NOTE: this is a cleaner way to solve but LeetCode doesn't support Go 1.21 right now
/**
import "cmp"

func search(nums []int, target int) int {
	left := 0
	right := len(nums) - 1
	for left <= right {
		mid := left + (right-left)/2
		switch cmp.Compare(nums[mid], target) {
		case 0:
			return mid
		case -1:
			left = mid + 1
		case 1:
			right = mid - 1
		}
	}
	return -1
}
*/
