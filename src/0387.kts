class Solution {
    fun firstUniqChar(s: String): Int {
        if (s.isEmpty()) {
            return -1
        }
        val dict = IntArray(26)
        for (i in s.indices) {
            dict[s[i] - 'a']++
        }
        for (i in s.indices) {
            if (dict[s[i] - 'a'] == 1) {
                return i
            }
        }
        return -1
    }
}
