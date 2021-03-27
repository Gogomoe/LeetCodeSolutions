class Solution {

    fun generateParenthesis(n: Int): List<String> {
        val result = mutableListOf<String>()
        dfs(result, "", 0, 0, n)
        return result
    }

    private fun dfs(result: MutableList<String>, str: String, level: Int, used: Int, n: Int) {
        if (level == 0 && used == n) {
            result.add(str)
            return
        }
        if (level > 0) {
            dfs(result, "$str)", level - 1, used, n)
        }
        if (used != n) {
            dfs(result, "$str(", level + 1, used + 1, n)
        }
    }

}