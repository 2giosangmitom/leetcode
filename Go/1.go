package main

import "fmt"

func twoSum(nums []int, target int) []int {
	var hashMap = make(map[int]int)
	for index, value := range nums {
		needNumber := target - value
		if _, ok := hashMap[needNumber]; ok {
			return []int{hashMap[needNumber], index}
		}
		hashMap[value] = index
	}
	return []int{-1}
}

func main() {
	var arr = []int{2, 7, 11, 15}
	fmt.Print(twoSum(arr, 9)) // TEST: => [0 1]
}
