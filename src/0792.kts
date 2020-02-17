class Solution {
    fun numMatchingSubseq(S: String, words: Array<String>): Int {
        val waiting = Array(26) { mutableSetOf<CharIterator>() }
        words.forEach {
            waiting[it[0] - 'a'].add(it.iterator().also { it.next() })
        }
        var result = 0
        for (c in S) {
            val toNext = waiting[c - 'a']
            waiting[c - 'a'] = mutableSetOf()
            for (iterator in toNext) {
                if (iterator.hasNext()) {
                    val next = iterator.next()
                    waiting[next - 'a'].add(iterator)
                } else {
                    result++
                }
            }
        }
        return result
    }
}