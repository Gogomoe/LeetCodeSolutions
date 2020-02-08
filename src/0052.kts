class Solution {
    var n = 0
    var column = BooleanArray(1)
    var skewLeft = BooleanArray(1)
    var skewRight = BooleanArray(1)
    var count = 0

    fun totalNQueens(n: Int): Int {
        this.n = n
        column = BooleanArray(n)
        skewLeft = BooleanArray(2 * n - 1)
        skewRight = BooleanArray(2 * n - 1)
        dfs(0)
        return count
    }

    private fun dfs(y: Int) {
        if (y == n) {
            count++
            return
        }
        for (x in 0 until n) {
            if (!column[x] && !skewLeft[x - y + n - 1] && !skewRight[x + y]) {
                column[x] = true
                skewLeft[x - y + n - 1] = true
                skewRight[x + y] = true
                dfs(y + 1)
                column[x] = false
                skewLeft[x - y + n - 1] = false
                skewRight[x + y] = false
            }
        }
    }
}