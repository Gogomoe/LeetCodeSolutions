class Solution {
    companion object {
        val KEYS: HashSet<Char> = HashSet(('1'..'9').toList())
    }

    fun solveSudoku(board: Array<CharArray>) {
        do {
            val count = fastSolve(board)
        } while (count != 0)

        dfs(board, Context.create().also { it.clear(board) })
    }

    private fun dfs(board: Array<CharArray>, context: Context): Boolean {
        for (i in 0 until 9) {
            for (j in 0 until 9) {
                val c = board[i][j]
                if (c == '.') {
                    val chooses = context.choose(i, j)
                    for (choose in chooses) {
                        context.rows[i].remove(choose)
                        context.cols[j].remove(choose)
                        context.square[i / 3][j / 3].remove(choose)
                        board[i][j] = choose
                        if (dfs(board, context)) {
                            return true
                        }
                        context.rows[i].add(choose)
                        context.cols[j].add(choose)
                        context.square[i / 3][j / 3].add(choose)
                        board[i][j] = '.'
                    }
                    return false
                }
            }
        }
        return true
    }

    private fun fastSolve(board: Array<CharArray>): Int {
        val context = Context.create()

        var count = 0
        for (i in 0 until 9) {
            for (j in 0 until 9) {
                val c = board[i][j]
                if (c == '.') {
                    val choose = context.choose(i, j)
                    if (choose.size == 1) {
                        board[i][j] = choose.first()
                        count++
                    }
                }
            }
        }

        return count
    }

    data class Context(
            val rows: List<HashSet<Char>>,
            val cols: List<HashSet<Char>>,
            val square: List<List<HashSet<Char>>>
    ) {
        companion object {
            fun create() = Context(
                    List(9) { HashSet(KEYS) },
                    List(9) { HashSet(KEYS) },
                    List(3) {
                        List(3) {
                            HashSet(KEYS)
                        }
                    }
            )
        }

        fun clear(board: Array<CharArray>) {
            for (i in 0 until 9) {
                for (j in 0 until 9) {
                    val c = board[i][j]
                    if (c != '.') {
                        rows[i].remove(c)
                        cols[j].remove(c)
                        square[i / 3][j / 3].remove(c)
                    }
                }
            }
        }

        fun choose(i: Int, j: Int): Set<Char> {
            val choose = rows[i].toMutableSet()
            choose.retainAll(cols[j])
            choose.retainAll(square[i / 3][j / 3])
            return choose
        }
    }

}