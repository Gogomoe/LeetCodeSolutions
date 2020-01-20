import kotlin.math.max

class Solution {

    fun checkRecord(n: Int): Int {
        val dp = Array(max(n, 3) + 1) { IntArray(2) }
        dp[1][0] = 2
        dp[1][1] = 1
        dp[2][0] = 4
        dp[2][1] = 4
        dp[3][0] = 7
        dp[3][1] = 12
        for (i in 4..n) {
            dp[i][0] = ((dp[i - 1][0] + dp[i - 2][0]) % m + dp[i - 3][0]) % m
            dp[i][1] = ((dp[i][0] + dp[i - 1][1]) % m + (dp[i - 2][1] + dp[i - 3][1]) % m) % m
        }
        return (dp[n][0] + dp[n][1]) % m
    }

    companion object {
        const val m = 1000000007
    }
}