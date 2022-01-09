class Solution {
    fun wordCount(startWords: Array<String>, targetWords: Array<String>): Int {
        val targets = targetWords.asSequence().map { hash(it) }.groupingBy { it }.eachCount().toMutableMap()
        var result = 0
        for (startWord in startWords) {
            val start = hash(startWord)
            for (i in 0..25) {
                if (start.and(1.shl(i)) == 0) {
                    val it = start.or(1.shl(i))
                    result += targets.remove(it) ?: 0
                }
            }
        }
        return result
    }

    private fun hash(str: String): Int {
        var hash = 0
        for (c in str) {
            hash = hash.or(1.shl(c - 'a'))
        }
        return hash
    }
}


println(Solution().wordCount(arrayOf("ant", "act", "tack"), arrayOf("tack", "act", "acti"))) // 2
println(Solution().wordCount(arrayOf("ab", "a"), arrayOf("abc", "abcd"))) // 1
println(
    Solution().wordCount(
        arrayOf("uh"),
        arrayOf("u", "hur", "k", "b", "u", "yse", "giqoy", "lni", "olqb", "nemc")
    )
) // 1