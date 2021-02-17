class Solution {
    fun largest1BorderedSquare(grid: Array<IntArray>): Int {
        val m = grid.size
        val n = grid[0].size
        val ups = Array(m) { IntArray(n) }
        val lefts = Array(m) { IntArray(n) }
        for (i in 0 until m) {
            for (j in 0 until n) {
                if (grid[i][j] == 1) {
                    lefts[i][j] = (lefts[i].getOrNull(j - 1) ?: 0) + 1
                    ups[i][j] = (ups.getOrNull(i - 1)?.get(j) ?: 0) + 1
                }
            }
        }
        var result = 0
        for (i in 0 until m) {
            for (j in 0 until n) {
                for (size in 0..minOf(i, j)) {
                    if (lefts[i][j] > size && ups[i][j - size] > size && ups[i][j] > size && lefts[i - size][j] > size) {
                        result = maxOf(result, size + 1)
                    }
                }
            }
        }

        return result * result
    }
}
