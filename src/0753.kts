import java.util.ArrayDeque

class Solution {
    fun crackSafe(n: Int, k: Int): String {
        if (n == 1) {
            return Array(k) { "$it" }.joinToString(separator = "")
        }
        val pows = mutableListOf(1)
        repeat(n) { pows.add(pows.last() * k) }

        val visited = BooleanArray(pows[n])
        val string = ArrayDeque<Char>()

        repeat(n) { string.addLast('0') }
        val hash = 0
        visited[hash] = true

        dfs(pows, n, k, visited, string, 1, hash)

        var it = 0
        while (string.size < pows[n] + n - 1) {
            string.add(string.elementAt(it++))
        }

        return string.joinToString(separator = "")
    }

    private fun dfs(pows: List<Int>, n: Int, k: Int, visited: BooleanArray, string: ArrayDeque<Char>, level: Int, hash: Int): Boolean {
        if (level == pows[n]) {
            return true
        }
        for (i in 0 until k) {
            val oldChar = string.elementAt(string.size - n)
            val newChar = '0' + i

            val newHash = (hash - (oldChar - '0') * pows[n - 1]) * k + i
            if (!visited[newHash]) {
                visited[newHash] = true
                string.addLast(newChar)

                val res = dfs(pows, n, k, visited, string, level + 1, newHash)
                if (res) {
                    return true
                }

                string.removeLast()
                visited[newHash] = false
            }
        }

        return false
    }
}