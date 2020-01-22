class Solution {
    fun numEnclaves(A: Array<IntArray>): Int {
        val w = A[0].size
        val h = A.size

        for (i in 0 until w) {
            paint(A, i, 0)
            paint(A, i, h - 1)
        }
        for (i in 0 until h) {
            paint(A, 0, i)
            paint(A, w - 1, i)
        }
        var count = 0
        for (y in 0 until h) {
            for (x in 0 until w) {
                if (A[y][x] == 1) {
                    count++
                }
            }
        }
        return count
    }

    private fun paint(A: Array<IntArray>, x: Int, y: Int) {
        val w = A[0].size
        val h = A.size

        if (x < 0 || x >= w || y < 0 || y >= h) {
            return
        }
        if (A[y][x] != 1) {
            return
        }
        A[y][x] = 2
        paint(A, x - 1, y)
        paint(A, x + 1, y)
        paint(A, x, y - 1)
        paint(A, x, y + 1)
    }
}