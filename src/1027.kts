class Solution {
    fun longestArithSeqLength(A: IntArray): Int {
        val dp = Array(501) { IntArray(1001) { 0 } }
        for (it in A) {
            for (diff in -500..500) {
                val index = diff + 500
                if (diff == 0) {
                    dp[it][index]++
                    continue
                }
                dp[it][index] = maxOf(dp[it][index], 1)
                val preNumber = it + diff
                if (preNumber < 0 || preNumber > 500 || dp[preNumber][index] == 0) {
                    continue
                }
                dp[it][index] = maxOf(dp[it][index], dp[preNumber][index] + 1)
            }
        }

        return dp.map { it.fold(0, ::maxOf) }.fold(0, ::maxOf)
    }
}