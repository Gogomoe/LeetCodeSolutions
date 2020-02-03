class Solution {
    class Tire {
        var present: String? = null
        val dict: MutableMap<Char, Tire> = mutableMapOf()

        fun insert(string: String) {
            var node = this
            for (c in string) {
                node = node.dict.getOrPut(c) { Tire() }
            }
            node.present = string
        }
    }

    fun findWords(board: Array<CharArray>, words: Array<String>): List<String> {
        if (board.isEmpty() || board[0].isEmpty()) {
            return emptyList()
        }
        val tire = Tire()
        words.forEach { tire.insert(it) }
        val result = mutableSetOf<String>()
        val height = board.size
        val width = board[0].size
        val used = Array(height) { BooleanArray(width) }
        fun dfs(x: Int, y: Int, tree: Tire?) {
            if (tree == null) {
                return
            }
            tree.present?.let { result.add(it) }
            listOf(x - 1 to y, x + 1 to y, x to y - 1, x to y + 1).filter { (itx, ity) ->
                itx in board[0].indices && ity in board.indices && !used[ity][itx]
            }.forEach { (itx, ity) ->
                used[ity][itx] = true
                dfs(itx, ity, tree.dict[board[ity][itx]])
                used[ity][itx] = false
            }
        }
        for (y in board.indices) {
            for (x in board[0].indices) {
                used[y][x] = true
                dfs(x, y, tire.dict[board[y][x]])
                used[y][x] = false
            }
        }
        return result.toList()
    }
}