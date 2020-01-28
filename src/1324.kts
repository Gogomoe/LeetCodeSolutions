class Solution {
    fun printVertically(s: String): List<String> {
        val ss = s.split(' ')
        val width = ss.map { it.length }.max()!!
        val cs = Array(width) { CharArray(ss.size) { ' ' } }
        for (i in ss.indices) {
            for (j in cs.indices) {
                cs[j][i] = ss[i].getOrElse(j) { ' ' }
            }
        }
        return cs.map { String(it).trimEnd() }
    }
}