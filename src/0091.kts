class Solution {
    fun numDecodings(s: String): Int {
        val chars: CharArray = ("$s ").toCharArray()
        val dp = IntArray(s.length + 2)
        dp[s.length] = 1
        for (i in s.indices.reversed()) {
            if (chars[i] != '0') {
                dp[i] += dp[i + 1]
            }
            if (chars[i] == '1' || (chars[i] == '2' && chars[i + 1] >= '0' && chars[i + 1] <= '6')) {
                dp[i] += dp[i + 2]
            }
        }
        return dp[0]
    }
}

println(Solution().numDecodings("12")) // 2
println(Solution().numDecodings("226")) // 3
println(Solution().numDecodings("0")) // 0
println(Solution().numDecodings("06")) // 0
println(Solution().numDecodings("1")) // 1