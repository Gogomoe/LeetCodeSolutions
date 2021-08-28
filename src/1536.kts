class Solution {
    fun minSwaps(grid: Array<IntArray>): Int {
        val n = grid.size
        val arr = mutableListOf<Int>()

        for (i in grid.indices) {
            val row = grid[i]

            var zeros = 0
            for (j in (n - 1) downTo 0) {
                if (row[j] == 0) {
                    zeros++
                } else {
                    break
                }
            }

            arr.add(zeros)
        }

        var cost = 0
        outer@ for (i in 0..n - 2) {
            val needZeros = n - i - 1
            for (j in arr.indices) {
                if (arr[j] >= needZeros) {
                    cost += j
                    arr.removeAt(j)
                    continue@outer
                }
            }
            return -1
        }

        return cost
    }
}