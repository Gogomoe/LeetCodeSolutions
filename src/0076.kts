class Solution {
    fun minWindow(s: String, t: String): String {
        if (t.isEmpty()) {
            return ""
        }
        val cs = s.toCharArray()
        val map = mutableMapOf<Char, Int>()
        t.forEach { map.merge(it, 1, Int::plus) }
        var cnt = map.keys.size
        var minL = 0
        var minR = 0
        var L = 0
        var R = 0
        var containAll = false
        while (true) {
            if (containAll) {
                val c = cs[L++]
                if (c in map) {
                    map[c] = map[c]!! + 1
                }
                if (map[c] == 1) {
                    cnt++
                }
                if (cnt > 0) {
                    containAll = false
                }
            } else {
                if (R == cs.size) {
                    break
                }
                val c = cs[R++]
                if (c in map) {
                    map[c] = map[c]!! - 1
                    if (map[c] == 0) {
                        cnt--
                    }
                    if (cnt == 0) {
                        containAll = true
                    }
                }
            }

            if (containAll && (R - L < minR - minL || minR == 0)) {
                minL = L
                minR = R
            }
        }
        return s.substring(minL, minR)
    }
}