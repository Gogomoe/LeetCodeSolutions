class Solution {
    fun rotate(matrix: Array<IntArray>): Unit {
        val n = matrix.size
        val row = n / 2
        val col = if (n % 2 == 0) row else row + 1
        for (i in 0 until row) {
            for (j in 0 until col) {
                val temp = matrix[i][j]
                matrix[i][j] = matrix[n - j - 1][i]
                matrix[n - j - 1][i] = matrix[n - i - 1][n - j - 1]
                matrix[n - i - 1][n - j - 1] = matrix[j][n - i - 1]
                matrix[j][n - i - 1] = temp
            }
        }

    }
}

val matrix1 = arrayOf(
    intArrayOf(1, 2, 3),
    intArrayOf(4, 5, 6),
    intArrayOf(7, 8, 9)
)
Solution().rotate(matrix1)
println(matrix1.map { it.contentToString() }) // [[7,4,1],[8,5,2],[9,6,3]]

val matrix2 = arrayOf(
    intArrayOf(5, 1, 9, 11),
    intArrayOf(2, 4, 8, 10),
    intArrayOf(13, 3, 6, 7),
    intArrayOf(15, 14, 12, 16),
)
Solution().rotate(matrix2)
println(matrix2.map { it.contentToString() }) // [[15,13,2,5],[14,3,4,1],[12,6,8,9],[16,7,10,11]]