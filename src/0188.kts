import java.lang.Math.max

class Solution {
    fun maxProfit(k: Int, prices: IntArray): Int {
        if (prices.size <= 1 || k == 0) {
            return 0
        }
        val len = prices.size
        val hold = Array(k + 1) { IntArray(prices.size) }
        val unhold = Array(k + 1) { IntArray(prices.size) }
        hold[0][0] = -prices[0]
        for (i in 1 until len) {
            hold[0][i] = max(hold[0][i - 1], -prices[i])
        }
        for (j in 1 until k) {
            hold[j][0] = hold[j - 1][0]
            for (i in 1 until len) {
                hold[j][i] = max(hold[j][i - 1], unhold[j][i - 1] - prices[i])
                unhold[j][i] = max(unhold[j][i - 1], hold[j - 1][i - 1] + prices[i])
            }
        }
        for (i in 1 until len) {
            unhold[k][i] = max(unhold[k][i - 1], hold[k - 1][i - 1] + prices[i])
        }
        return unhold[k][len - 1]
    }
}

println(Solution().maxProfit(2, intArrayOf(2, 4, 1))) // 2
println(Solution().maxProfit(2, intArrayOf(3, 2, 6, 5, 0, 3))) // 7
println(Solution().maxProfit(2, intArrayOf())) // 0
println(Solution().maxProfit(0, intArrayOf(1, 3))) // 0