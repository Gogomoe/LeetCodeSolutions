class Solution {
    fun checkValid(matrix: Array<IntArray>): Boolean {
        val n = matrix.size
        val set = mutableSetOf<Int>()
        for (i in 0 until n) {
            set.clear()
            for (j in 0 until n) {
                if (!set.add(matrix[j][i])) {
                    return false
                }
            }
        }
        for (i in 0 until n) {
            set.clear()
            for (j in 0 until n) {
                if (!set.add(matrix[i][j])) {
                    return false
                }
            }
        }
        return true
    }
}

println(
    Solution().checkValid(
        arrayOf(
            intArrayOf(1, 2, 3),
            intArrayOf(3, 1, 2),
            intArrayOf(2, 3, 1),
        )
    )
) // true

println(
    Solution().checkValid(
        arrayOf(
            intArrayOf(1, 1, 1),
            intArrayOf(1, 2, 2),
            intArrayOf(1, 2, 3),
        )
    )
) // false