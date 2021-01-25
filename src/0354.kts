class Solution {
    fun maxEnvelopes(envelopes: Array<IntArray>): Int {
        if (envelopes.isEmpty()) {
            return 0
        }
        envelopes.sortWith(Comparator { (x1, y1), (x2, y2) ->
            if (x1 != x2) {
                x1 - x2
            } else {
                y2 - y1
            }
        })

        val dp = IntArray(envelopes.size)
        var size = 0

        dp[0] = envelopes[0][1]
        size++

        for (it in envelopes) {
            val y = it[1]

            var L = 0
            var R = size
            while (L != R) {
                val mid = (L + R) / 2
                if (dp[mid] >= y) {
                    R = mid
                } else {
                    L = mid + 1
                }
            }

            dp[L] = y
            if (L == size) {
                size++
            }
        }
        return size
    }
}
