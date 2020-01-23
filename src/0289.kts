class Solution {

    private lateinit var board: Array<IntArray>
    private var h: Int = 0
    private var w: Int = 0

    fun gameOfLife(board: Array<IntArray>): Unit {
        this.board = board
        h = board.size
        w = board[0].size

        val res = Array(h) { IntArray(w) }
        for (i in 0 until h) {
            for (j in 0 until w) {
                val it = board[i][j] == 1
                val lives = countLiveNeighbor(j, i)
                res[i][j] = when {
                    it && lives < 2 -> 0
                    it && lives in 2..3 -> 1
                    it && lives > 3 -> 0
                    !it && lives == 3 -> 1
                    else -> 0
                }
            }
        }
        for (i in 0 until h) {
            for (j in 0 until w) {
                board[i][j] = res[i][j]
            }
        }

    }

    private val near = listOf(
        -1 to -1,
        -1 to 0,
        -1 to 1,
        0 to -1,
        0 to 1,
        1 to -1,
        1 to 0,
        1 to 1
    )

    private fun countLiveNeighbor(x: Int, y: Int): Int {
        return near.count { (i, j) ->
            val itx = i + x
            val ity = j + y
            itx in 0 until w && ity in 0 until h && board[ity][itx] == 1
        }
    }
}