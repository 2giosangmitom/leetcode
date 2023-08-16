/**
 * Runtime: 6ms (Beats 83.61%)
 * Memory: 4.4MB (Beats 98.13%)
 */

package removeduplicatesfromsortedarr

func removeDuplicates(nums []int) int {
	k := 0
	for i := 1; i < len(nums); i++ {
		if nums[i] != nums[k] {
			k++
			nums[k] = nums[i]
		}
	}
	return k + 1
}
