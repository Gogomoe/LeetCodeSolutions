class Solution {
    fun colorBorder(grid: Array<IntArray>, r0: Int, c0: Int, color: Int): Array<IntArray> {
        if (grid.isEmpty() || grid[0].isEmpty()) {
            return grid
        }
        val rows = grid.size
        val columns = grid[0].size

        val visited = Array(rows) { BooleanArray(columns) }
        val toPaint = mutableListOf<Pair<Int, Int>>()

        dfs(grid, visited, toPaint, grid[r0][c0], r0, c0)

        for ((r, c) in toPaint) {
            grid[r][c] = color
        }

        return grid
    }

    private fun dfs(
            grid: Array<IntArray>, visited: Array<BooleanArray>,
            toPaint: MutableList<Pair<Int, Int>>, color: Int, r0: Int, c0: Int
    ): Boolean {
        val rows = grid.size
        val columns = grid[0].size
        if (r0 < 0 || r0 >= rows || c0 < 0 || c0 >= columns || grid[r0][c0] != color) {
            return true
        }
        if (visited[r0][c0]) {
            return false
        }

        visited[r0][c0] = true
        if (
                listOf(
                        dfs(grid, visited, toPaint, color, r0 - 1, c0),
                        dfs(grid, visited, toPaint, color, r0 + 1, c0),
                        dfs(grid, visited, toPaint, color, r0, c0 - 1),
                        dfs(grid, visited, toPaint, color, r0, c0 + 1)
                ).any { it }
        ) {
            toPaint.add(r0 to c0)
        }

        return false
    }
}