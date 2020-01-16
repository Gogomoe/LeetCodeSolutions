import kotlin.math.max

class Solution {
    fun maxCoins(nums: IntArray): Int {
        val n = nums.size
        val list = listOf(1, *nums.toTypedArray(), 1)
        val dp = Array(nums.size + 2) { IntArray(nums.size + 2) }
        for (len in 1..n) {
            for (left in 1..(n - len + 1)) {
                val right = left + len - 1

                var maxCoins = 0

                /**
                 * i 表示最后区间内最后一个戳爆气球的位置，因此乘的是 `list[left - 1]`, `list[right + 1]`
                 */
                for (i in left..right) {
                    val coins = list[left - 1] * list[i] * list[right + 1] + dp[left][i - 1] + dp[i + 1][right]
                    maxCoins = max(maxCoins, coins)
                }
                dp[left][right] = maxCoins

            }
        }
        return dp[1][n]
    }

}

println(Solution().maxCoins(intArrayOf(3, 2, 5)))