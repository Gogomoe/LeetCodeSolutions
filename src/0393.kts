import java.lang.IllegalStateException

class Solution {
    fun validUtf8(data: IntArray): Boolean {
        var pos = 0
        while (pos < data.size) {
            try {
                pos = read(pos, data)
            } catch (e: IllegalStateException) {
                return false
            }
        }
        return true
    }

    private fun read(pos: Int, data: IntArray): Int {
        var p = pos
        val first = data[p].also { p++ }
        val byteLen = when {
            !first[8] -> 0
            first[8] && first[7] && !first[6] -> 1
            first[8] && first[7] && first[6] && !first[5] -> 2
            first[8] && first[7] && first[6] && first[5] && !first[4] -> 3
            else -> throw IllegalStateException()
        }
        repeat(byteLen) {
            check(p < data.size)
            val byte = data[p].also { p++ }
            check((byte[8] && !byte[7]))
        }
        return p
    }

    private operator fun Int.get(pos: Int): Boolean {
        return this and (1 shl (pos - 1)) != 0
    }
}