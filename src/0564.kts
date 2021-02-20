import kotlin.math.abs

class Solution {
    fun nearestPalindromic(n: String): String {
        val candidates = mutableListOf<String>()

        val chars = n.toCharArray()
        val mid = (n.length - 1) / 2
        val midChar = chars[mid]

        if (n.length > 1) {
            candidates.add(String(CharArray(n.length - 1) { '9' }))
        }

        if (midChar != '0') {
            chars[mid] = midChar - 1
            candidates.add(makePalindrome(chars))
        }
        chars[mid] = midChar
        val palindrome = makePalindrome(chars)
        if (n != palindrome) {
            candidates.add(palindrome)
        }
        if (midChar != '9') {
            chars[mid] = midChar + 1
            candidates.add(makePalindrome(chars))
        }

        candidates.add(String(CharArray(n.length + 1) {
            when (it) {
                0, n.length -> '1'
                else -> '0'
            }
        }))

        var result = candidates.first()
        val origin = n.toLong()
        for (str in candidates) {
            if (abs(str.toLong() - origin) < abs(result.toLong() - origin)) {
                result = str
            }
        }
        return result
    }

    private fun makePalindrome(chars: CharArray): String {
        val clone = chars.clone()
        var i = 0
        var j = clone.size - 1
        while (i < j) {
            clone[j--] = clone[i++]
        }
        return String(clone)
    }
}