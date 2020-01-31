class Solution {

    fun islandPerimeter(grid: Array<IntArray>): Int {
        fun getCell(i: Int, j: Int): Int {
            return if (i !in grid.indices || j !in grid[0].indices) 0 else grid[i][j]
        }

        var cnt = 0
        for (i in grid.indices) {
            for (j in grid[i].indices) {
                if (grid[i][j] == 1) {
                    cnt += 1 - getCell(i, j - 1)
                    cnt += 1 - getCell(i, j + 1)
                    cnt += 1 - getCell(i - 1, j)
                    cnt += 1 - getCell(i + 1, j)
                }
            }
        }
        return cnt
    }

}