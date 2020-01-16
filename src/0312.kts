import kotlin.math.max

class Solution {
    fun maxCoins(nums: IntArray): Int {
        val dp = Array(nums.size) { IntArray(nums.size) }
        val res = maxCoinsInRange(nums, dp, 0, nums.size - 1)
        return res
    }

    private fun maxCoinsInRange(nums: IntArray, dp: Array<IntArray>, left: Int, right: Int): Int {
        if (left > right) return 0
//        if (left == right) return nums[left]
        if (dp[left][right] != 0) return dp[left][right]

        var maxCoins = 0
        /**
         * i 表示最后区间内最后一个戳爆气球的位置，因此乘的是 `nums[left - 1]`, `nums[right + 1]`
         * `maxCoinsInRange` 会考虑到 `left - 1` 和 `right + 1` 位置的气球
         * 上方注释位置应该改成 `nums[left - 1] * nums[left] * nums[left + 1]`
         */
        for (i in left..right) {
            val coins = nums.getOrElse(left - 1) { 1 } * nums[i] * nums.getOrElse(right + 1) { 1 }
            val sum = coins + maxCoinsInRange(nums, dp, left, i - 1) + maxCoinsInRange(nums, dp, i + 1, right)
            maxCoins = max(maxCoins, sum)
        }
        dp[left][right] = maxCoins
        return maxCoins
    }
}

println(Solution().maxCoins(intArrayOf(3, 2, 5)))