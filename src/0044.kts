class Solution {
    fun isMatch(s: String, p: String): Boolean {
        var dp = BooleanArray(s.length + 1)
        var next = BooleanArray(s.length + 1)
        dp[0] = true
        for (i in 1..p.length) {
            val patternChar = p[i - 1]
            next[0] = dp[0] && patternChar == '*'

            var meetStar = dp[0] && patternChar == '*'
            for (j in 1..s.length) {
                val stringChar = s[j - 1]
                meetStar = meetStar || (dp[j] && patternChar == '*')
                if (meetStar) {
                    next[j] = true
                    continue
                }
                next[j] = (patternChar == '?' || patternChar == stringChar) && dp[j - 1]
            }

            val temp = dp
            dp = next
            next = temp
        }
        return dp[s.length]
    }
}