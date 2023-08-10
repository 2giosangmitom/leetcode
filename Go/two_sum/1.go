/**
 * Runtime: 7ms (Beats 70.84%)
 * Memory: 4.3MB (Beats 13.87%)
 */

package twosum

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)
	for i, num := range nums {
		needNumber := target - num
		if _, ok := hashMap[needNumber]; ok {
			return []int{hashMap[needNumber], i}
		} else {
			hashMap[num] = i
		}
	}
	return []int{-1}
}
