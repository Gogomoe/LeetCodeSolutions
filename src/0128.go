package main

func longestConsecutive(nums []int) int {
	lens := map[int]int{}
	res := 0
	for _, num := range nums {
		if _, ok := lens[num]; ok {
			continue
		}
		leftNum := num
		rightNum := num
		if _, ok := lens[num-1]; ok {
			leftNum = num - lens[num-1]
		}
		if _, ok := lens[num+1]; ok {
			rightNum = num + lens[num+1]
		}
		newLen := rightNum - leftNum + 1
		// only update the boundary of the lens
		lens[num] = newLen
		lens[leftNum] = newLen
		lens[rightNum] = newLen
		res = max(res, newLen)
	}
	return res
}
