class Solution {
    fun minDistance(word1: String, word2: String): Int {
        var last = IntArray(word2.length + 1)
        var now = IntArray(word2.length + 1)
        for (i in 0..word2.length) {
            last[i] = i
        }
        for (i in word1.indices) {
            now[0] = i + 1
            for (j in word2.indices) {
                now[j + 1] = minOf(last[j + 1] + 1, last[j] + 1, now[j] + 1)
                if (word1[i] == word2[j]) {
                    now[j + 1] = minOf(now[j + 1], last[j])
                }
            }
            val temp = now
            now = last
            last = temp
        }
        return last[word2.length]
    }
}