import kotlin.math.min

typealias Str = Map<Char, Int>

class Solution {

    private fun from(str: String): Str {
        return str.toList().groupingBy { it }.eachCount()
    }

    private fun fromWithAlphabet(str: String, alphabet: Str): Str {
        return str.toList().filter { alphabet.getOrDefault(it, 0) != 0 }.groupingBy { it }.eachCount()
    }

    private fun Str.dominate(other: Str): Boolean {
        for ((char, count) in other) {
            if (this.getOrDefault(char, 0) < count) {
                return false
            }
        }
        return true
    }

    fun minStickers(stickers: Array<String>, target: String): Int {
        val alphabet = from(target)
        val usableStrs = mutableListOf<Str>()
        val stickerStrs = stickers.map { fromWithAlphabet(it, alphabet) }
        outer@ for (i in stickerStrs) {
            for (j in stickerStrs) {
                if (i != j && j.dominate(i)) {
                    continue@outer
                }
            }
            usableStrs.add(i)
        }
        dfs(usableStrs, mutableMapOf(), 0, 0, target)
        return if (minSticker == Int.MAX_VALUE) -1 else minSticker
    }

    private fun dfs(
        strs: List<Str>,
        used: MutableMap<Char, Int>,
        pos: Int,
        cnt: Int,
        target: String
    ) {
        if (cnt >= minSticker) {
            return
        }
        if (pos == target.length) {
            minSticker = min(cnt, minSticker)
            return
        }
        val char = target[pos]
        if (used.getOrDefault(char, 0) > 0) {
            used.merge(char, 1, Int::minus)
            dfs(strs, used, pos + 1, cnt, target)
            used.merge(char, 1, Int::plus)
        } else {
            for (str in strs) {
                if (!str.containsKey(char)) {
                    continue
                }
                str.forEach { (c, i) -> used.merge(c, i, Int::plus) }
                used.merge(char, 1, Int::minus)
                dfs(strs, used, pos + 1, cnt + 1, target)
                used.merge(char, 1, Int::plus)
                str.forEach { (c, i) -> used.merge(c, i, Int::minus) }
            }
        }
    }

    var minSticker = Int.MAX_VALUE
}
