class Solution {
    fun spiralOrder(matrix: Array<IntArray>): List<Int> {
        if (matrix.isEmpty() || matrix[0].isEmpty()) {
            return emptyList()
        }
        var n = matrix.size
        var m = matrix[0].size
        var y = 0
        var x = -1
        var i = 0
        val res = MutableList(m * n) { 0 }

        n--
        while (m > 0 || n > 0) {
            if (m <= 0) break
            repeat(m) {
                x++
                res[i++] = matrix[y][x]
            }
            m--
            if (n <= 0) break
            repeat(n) {
                y++
                res[i++] = matrix[y][x]
            }
            n--
            if (m <= 0) break
            repeat(m) {
                x--
                res[i++] = matrix[y][x]
            }
            m--
            if (n <= 0) break
            repeat(n) {
                y--
                res[i++] = matrix[y][x]
            }
            n--
        }
        return res
    }
}