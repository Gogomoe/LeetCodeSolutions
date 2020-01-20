class Solution {
    fun canMakePaliQueries(s: String, queries: Array<IntArray>): BooleanArray {
        val chars = s.toCharArray()
        val size = s.length
        val res = BooleanArray(queries.size)

        val acc = Array(size + 1) { IntArray(26) }
        for (i in 1..size) {
            for (j in 0..25) {
                acc[i][j] = acc[i - 1][j]
            }
            acc[i][chars[i - 1] - 'a']++
        }
        val frequency = IntArray(26)

        for (it in queries.indices) {
            val (left, right, k) = queries[it]
            for (j in 0..25) {
                frequency[j] = acc[right + 1][j] - acc[left][j]
            }

            val cost = frequency.count { it % 2 == 1 }
            res[it] = cost <= 2 * k + if ((right - left) % 2 == 0) 1 else 0
        }
        return res
    }
}

Solution().canMakePaliQueries("abcde", arrayOf(intArrayOf(0, 3, 2)))