class Solution {
    fun shortestBridge(A: Array<IntArray>): Int {
        val m = A.size
        val n = A[0].size

        val queue = ArrayDeque<Pair<Int, Int>>()
        val bfs = ArrayDeque<Pair<Int, Int>>()
        out@ for (i in A.indices) {
            for (j in A[i].indices) {
                if (A[i][j] == 1) {
                    queue.add(i to j)
                    A[i][j] = 2
                    break@out
                }
            }
        }
        while (queue.isNotEmpty()) {
            val (i, j) = queue.removeFirst()
            var inside = true
            for ((y, x) in listOf(0 to 1, 0 to -1, 1 to 0, -1 to 0)) {
                val ny = i + y
                val nx = j + x
                if (ny < 0 || ny >= m || nx < 0 || nx >= n) {
                    continue
                }
                when (A[ny][nx]) {
                    1 -> {
                        A[ny][nx] = 2
                        queue.addLast(ny to nx)
                    }
                    0 -> inside = false
                }
            }
            if (!inside) {
                bfs.addLast(i to j)
            }
        }

        while (bfs.isNotEmpty()) {
            val (i, j) = bfs.removeFirst()
            for ((y, x) in listOf(0 to 1, 0 to -1, 1 to 0, -1 to 0)) {
                val ny = i + y
                val nx = j + x
                if (ny < 0 || ny >= m || nx < 0 || nx >= n) {
                    continue
                }
                when (A[ny][nx]) {
                    0 -> {
                        A[ny][nx] = A[i][j] + 1
                        bfs.addLast(ny to nx)
                    }
                    1 -> {
                        return A[i][j] - 2
                    }
                }
            }
        }
        return -1
    }
}
