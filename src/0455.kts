class Solution {
    fun findContentChildren(g: IntArray, s: IntArray): Int {
        var result = 0
        g.sortDescending()
        s.sortDescending()

        var j = 0
        for (child in g) {
            if (j == s.size) break
            if (child <= s[j]) {
                j++
                result++
            }
        }

        return result
    }
}