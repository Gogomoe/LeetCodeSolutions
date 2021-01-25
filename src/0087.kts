class Solution {
    fun isScramble(s1: String, s2: String): Boolean {
        val length = s1.length
        if (s1 == s2) {
            return true
        }
        if (s1.groupingBy { it }.eachCount() != s2.groupingBy { it }.eachCount()) {
            return false
        }
        for (i in 1 until length) {
            if (isScramble(s1.substring(0, i), s2.substring(0, i)) &&
                    isScramble(s1.substring(i, length), s2.substring(i, length))) {
                return true
            }
            if (isScramble(s1.substring(0, i), s2.substring(length - i, length)) &&
                    isScramble(s1.substring(i, length), s2.substring(0, length - i))) {
                return true
            }
        }
        return false
    }

}