/**
 * Runtime: 2ms (Beats 63.46%)
 * Memory: 2.14MB (Beats 9.66%)
 */

package remove_element

func removeElement(nums []int, val int) int {
	i := 0
	for _, v := range nums {
		if v != val {
			nums[i] = v
			i++
		}
	}
	return i
}
