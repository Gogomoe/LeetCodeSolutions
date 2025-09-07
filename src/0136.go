package main

func singleNumber(nums []int) int {
	res := 0
	for _, num := range nums {
		res = res ^ num
	}
	return res
}
