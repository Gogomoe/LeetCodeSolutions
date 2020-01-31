import java.lang.StringBuilder

class Solution {
    fun findReplaceString(S: String, indexes: IntArray, sources: Array<String>, targets: Array<String>): String {
        val list = indexes.indices.map {
            Triple(indexes[it], sources[it], targets[it])
        }.sortedBy { it.first }
        val chars = S.toCharArray()
        var i = 0
        var j = 0
        val result = StringBuilder()
        while (i < list.size) {
            val (index, source, target) = list[i]
            while (j < index) {
                result.append(chars[j])
                j++
            }
            if (match(chars, j, source)) {
                target.forEach { result.append(it) }
                j += source.length
            }
            i++
        }
        while (j < chars.size) {
            result.append(chars[j])
            j++
        }
        return result.toString()
    }

    private fun match(chars: CharArray, offset: Int, source: String): Boolean {
        if (chars.size - offset < source.length) {
            return false
        }
        for (index in source.indices) {
            if (chars[offset + index] != source[index]) {
                return false
            }
        }
        return true
    }
}