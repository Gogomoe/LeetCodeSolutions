import kotlin.math.max

class Solution {
    fun maxSumSubmatrix(matrix: Array<IntArray>, k: Int): Int {
        val height = matrix.size
        val width = matrix[0].size
        val arr = Array(height) { IntArray(width) }
        var max = Int.MIN_VALUE

        for (fromY in matrix.indices) {
            for (fromX in matrix[fromY].indices) {
                for (toY in fromY until height) {
                    for (toX in fromX until width) {
                        arr[toY][toX] = when {
                            toX == fromX && toY == fromY -> matrix[toY][toX]
                            toX == fromX -> arr[toY - 1][toX] + matrix[toY][toX]
                            toY == fromY -> arr[toY][toX - 1] + matrix[toY][toX]
                            else -> arr[toY - 1][toX] + arr[toY][toX - 1] - arr[toY - 1][toX - 1] + matrix[toY][toX]
                        }
                        if (arr[toY][toX] <= k) max = max(max, arr[toY][toX])
                    }
                }
            }
        }
        return max
    }
}