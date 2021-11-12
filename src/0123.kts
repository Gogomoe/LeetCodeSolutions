import java.lang.Math.max
import java.lang.Math.min

class Solution {
    fun maxProfit(prices: IntArray): Int {
        val leftProfit = IntArray(prices.size)
        var minPrice = prices[0]
        var maxProfit = 0
        for (i in prices.indices) {
            minPrice = min(minPrice, prices[i])
            maxProfit = max(maxProfit, prices[i] - minPrice)
            leftProfit[i] = maxProfit
        }
        val rightProfit = IntArray(prices.size)
        var maxPrice = prices[prices.size - 1]
        maxProfit = 0
        for (i in prices.indices.reversed()) {
            maxPrice = max(maxPrice, prices[i])
            maxProfit = max(maxProfit, maxPrice - prices[i])
            rightProfit[i] = maxProfit
        }

        var result = 0
        for (i in prices.indices) {
            result = max(result, leftProfit[i] + rightProfit[i])
        }
        return result
    }
}

println(Solution().maxProfit(intArrayOf(3, 3, 5, 0, 0, 3, 1, 4))) // 6
println(Solution().maxProfit(intArrayOf(1, 2, 3, 4, 5))) // 4
println(Solution().maxProfit(intArrayOf(7, 6, 4, 3, 1))) // 0
println(Solution().maxProfit(intArrayOf(1))) // 0
println(Solution().maxProfit(intArrayOf(6, 1, 3, 2, 4, 7))) // 7