package main

func maxSubArray(nums []int) int {
	dp := make([]int, len(nums))
	res := nums[0]
	dp[0] = nums[0]
	for i := 1; i < len(nums); i++ {
		dp[i] = max(nums[i], nums[i]+dp[i-1])
		res = max(res, dp[i])
	}
	return res
}
