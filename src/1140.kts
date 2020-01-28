import kotlin.math.max

class Solution {
    fun stoneGameII(piles: IntArray): Int {
        val n = piles.size
        val sums = IntArray(n)
        var sum = 0
        for (i in piles.indices.reversed()) {
            sum += piles[i]
            sums[i] = sum
        }

        val dp = mutableListOf<IntArray>()
        fun helper(pos: Int, m: Int): Int {
            if (pos > n) {
                return 0
            }
            if (pos + 2 * m >= n) {
                return sums[pos]
            }
            repeat(m - dp.size + 1) {
                dp.add(IntArray(n))
            }

            if (dp[m][pos] != 0) {
                return dp[m][pos]
            }

            var res = 0
            for (i in 1..2 * m) {
                res = max(res, sums[pos] - helper(pos + i, max(i, m)))
            }
            dp[m][pos] = res

            return res
        }
        return helper(0, 1)
    }
}