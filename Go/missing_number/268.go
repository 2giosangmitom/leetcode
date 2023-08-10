/**
 * Runtime: 14ms (Beats 82.73%)
 * Memory: 6.2MB (Beats 94.24%)
 */

package missingnumber

func missingNumber(nums []int) int {
	l := len(nums)
	sum := 0
	total := (l * (l + 1)) / 2
	for _, num := range nums {
		sum += num
	}
	return total - sum
}
