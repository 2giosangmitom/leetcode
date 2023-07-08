package main

import "fmt"

func twoSum(nums []int, target int) []int {
	hashMap := make(map[int]int)
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
	arr1 := []int{2, 7, 11, 15}
	fmt.Println(twoSum(arr1, 9)) // TEST: => [0 1]
	arr2 := []int{3, 2, 4}
	fmt.Println(twoSum(arr2, 6)) // TEST: => [1 2]
}
