class Solution {
    fun minDays(grid: Array<IntArray>): Int {
        val lands = grid.map { row -> row.count { it == 1 } }.sum()
        if (!isConnected(grid, lands)) {
            return 0
        }

        val rows = grid.size
        val cols = grid[0].size
        for (i in 0 until rows) {
            for (j in 0 until cols) {
                if (grid[i][j] == 1) {
                    grid[i][j] = 0
                    if (!isConnected(grid, lands - 1)) {
                        return 1
                    }
                    grid[i][j] = 1
                }
            }
        }

        return 2
    }

    private fun isConnected(grid: Array<IntArray>, expected: Int): Boolean {
        val rows = grid.size
        val cols = grid[0].size
        val visited = BooleanArray(rows * cols)
        val queue = ArrayDeque<Int>()
        var count = 0

        fun addNode(y: Int, x: Int) {
            if (y < 0 || y >= rows || x < 0 || x >= cols) {
                return
            }
            if (visited[y * cols + x] || grid[y][x] == 0) {
                return
            }
            queue.addLast(y * cols + x)
            visited[y * cols + x] = true
            count++
        }

        out@ for (i in 0 until rows) {
            for (j in 0 until cols) {
                if (grid[i][j] == 1) {
                    addNode(i, j)
                    break@out
                }
            }
        }

        while (queue.isNotEmpty()) {
            val it = queue.removeFirst()
            val i = it / cols
            val j = it % cols
            addNode(i + 1, j)
            addNode(i - 1, j)
            addNode(i, j + 1)
            addNode(i, j - 1)
        }

        if (count == 0) {
            return false
        }

        return count == expected
    }
}