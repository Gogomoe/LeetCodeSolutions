class Solution {
    fun isInterleave(s1: String, s2: String, s3: String): Boolean {
        if (s1.length + s2.length != s3.length) {
            return false
        }
        var dp = BooleanArray(s1.length + 1)
        var new = BooleanArray(s1.length + 1)

        dp[0] = true
        for (i in s1.indices) {
            val s1Len = i + 1
            if (s1[s1Len - 1] == s3[s1Len - 1]) {
                dp[s1Len] = true
            } else {
                break
            }
        }

        for (i in s2.indices) {
            val s2Len = i + 1
            new[0] = dp[0] && s2[i] == s3[i]
            for (j in s1.indices) {
                val s1Len = j + 1
                new[s1Len] = (dp[s1Len] && s2[s2Len - 1] == s3[s2Len + s1Len - 1])
                        || (new[s1Len - 1] && s1[s1Len - 1] == s3[s2Len + s1Len - 1])
            }

            val temp = dp
            dp = new
            new = temp
        }

        return dp[s1.length]
    }
}