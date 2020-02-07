import kotlin.math.max

class Solution {
    fun projectionArea(grid: Array<IntArray>): Int {
        var res = 0
        val front = mutableListOf<Int>()
        for (i in grid.indices) {
            var line = 0
            for (j in grid[i].indices) {
                val it = grid[i][j]
                if (front.size == j) {
                    front.add(it)
                } else {
                    front[j] = max(front[j], it)
                }
                line = max(line, it)
                if (it != 0) {
                    res++
                }
            }
            res += line
        }
        return front.sum() + res
    }
}