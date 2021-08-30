import java.lang.Math.max

class Solution {
    fun longestPalindromeSubseq(s: String): Int {
        val r = s.reversed()
        val arr = Array(s.length + 1) { IntArray(s.length + 1) }

        for (i in s.indices) {
            for (j in r.indices) {
                if (s[i] == r[j]) {
                    arr[i + 1][j + 1] = arr[i][j] + 1
                } else {
                    arr[i + 1][j + 1] = max(arr[i + 1][j], arr[i][j + 1])
                }
            }
        }
        return arr[s.length][s.length]
    }
}

println(Solution().longestPalindromeSubseq("bbbab"))