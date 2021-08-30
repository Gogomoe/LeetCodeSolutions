import kotlin.math.max

class Solution {
    fun longestPalindrome(word1: String, word2: String): Int {
        val s = word1 + word2
        val n = s.length

        val arr = Array(n) { IntArray(n) }
        var ans = 0

        for (len in 1..n) {
            for (i in 0..n - len) {
                if (len == 1) {
                    arr[i][i] = 1
                    continue
                }
                if (s[i] == s[i + len - 1]) {
                    arr[i][i + len - 1] = arr[i + 1][i + len - 2] + 2
                    if (i < word1.length && i + len - 1 >= word1.length) {
                        ans = max(ans, arr[i][i + len - 1])
                    }
                } else {
                    arr[i][i + len - 1] = max(arr[i + 1][i + len - 1], arr[i][i + len - 2])
                }
            }
        }

        return ans
    }

}

println(
    Solution().longestPalindrome(
        "c",
        "fffaeacefcefeecedeedefecdcbedebebcfadffeacddcffa"
    )
)