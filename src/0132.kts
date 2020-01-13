import kotlin.math.min

class Solution {
    fun minCut(s: String): Int {
        val str = s.toCharArray()

        val isPalindrome: Array<BooleanArray> = generatePalindromeArray(str)

        val dp = IntArray(str.size)
        for (i in 1 until str.size) {
            dp[i] = i
            for (j in 0..i) {
                if (isPalindrome[j][i]) {
                    dp[i] = if (j == 0) 0 else min(dp[i], dp[j - 1] + 1)
                }
            }
        }
        return dp[str.size - 1]
    }

    /**
     * `isPalindrome[j][i]` == true for j <= i means str.substring(j..i) is palindrome
     */
    private fun generatePalindromeArray(str: CharArray): Array<BooleanArray> {
        val isPalindrome: Array<BooleanArray> = Array(str.size) { BooleanArray(str.size) }
        for (center in str.indices) {
            var left = center
            var right = center
            while (left >= 0 && right < str.size && str[left] == str[right]) {
                isPalindrome[left][right] = true
                left--
                right++
            }
        }
        for (center in 1 until str.size) {
            var left = center - 1
            var right = center
            while (left >= 0 && right < str.size && str[left] == str[right]) {
                isPalindrome[left][right] = true
                left--
                right++
            }
        }
        return isPalindrome
    }

}

println(Solution().minCut("aab"))

