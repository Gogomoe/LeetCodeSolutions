import java.util.*
import kotlin.math.max

class Solution {
    fun maxSumSubmatrix(matrix: Array<IntArray>, k: Int): Int {
        val height = matrix.size
        val width = matrix[0].size
        val partSum = Array(height) { IntArray(width) }
        var max = Int.MIN_VALUE

        for (toY in matrix.indices) {
            for (toX in matrix[0].indices) {
                partSum[toY][toX] = when {
                    toX == 0 && toY == 0 -> matrix[toY][toX]
                    toX == 0 -> partSum[toY - 1][toX] + matrix[toY][toX]
                    toY == 0 -> partSum[toY][toX - 1] + matrix[toY][toX]
                    else -> partSum[toY - 1][toX] + partSum[toY][toX - 1] - partSum[toY - 1][toX - 1] + matrix[toY][toX]
                }
            }
        }

        for (fromY in matrix.indices) {
            for (toY in fromY until height) {
                val arr = IntArray(width)
                for (x in matrix[0].indices) {
                    arr[x] = partSum[toY][x] - if (fromY == 0) 0 else partSum[fromY - 1][x]
                }
                max = max(max, maxSumSubArray(arr, k))
            }
        }
        return max
    }

    private fun maxSumSubArray(arr: IntArray, k: Int): Int {
        val set = TreeSet<Int>()
        set.add(0)
        var result = Int.MIN_VALUE
        for (it in arr) {
            val target = set.ceiling(it - k)
            if (target != null && it - target <= k) {
                result = max(result, it - target)
            }
            set.add(it)
        }
        return result
    }
}
