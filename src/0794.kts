class Solution {
    fun validTicTacToe(board: Array<String>): Boolean {
        var countX = 0
        var countO = 0
        for (row in board) {
            for (it in row) {
                when (it) {
                    'X' -> countX++
                    'O' -> countO++
                }
            }
        }
        return (countX == countO && !checkWin(board, 'X')) || (countX == countO + 1 && !checkWin(board, 'O'))
    }

    private fun checkWin(board: Array<String>, c: Char): Boolean {
        var oblique1 = 0
        var oblique2 = 0
        for (i in 0..2) {
            var rowCount = 0
            var colCount = 0
            for (j in 0..2) {
                if (board[i][j] == c) rowCount++
                if (board[j][i] == c) colCount++
            }
            if (board[i][i] == c) oblique1++
            if (board[i][2-i] == c) oblique2++

            if (rowCount == 3 || colCount == 3) {
                return true
            }
        }
        return oblique1 == 3 || oblique2 == 3
    }
}